use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn run(args: &[String]) -> Result<(), String> {
    if matches!(args.first().map(String::as_str), Some("-h" | "--help")) {
        print_usage();
        return Ok(());
    }
    let mut args = args.iter();

    let root_dir = git_toplevel()?;
    let out_dir = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| root_dir.join("out/orchestrator_linked_ai"));
    let repo_root = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| root_dir.clone());
    let goal = args.next().cloned().unwrap_or_else(|| {
        "Inspect the repository and produce a safe implementation plan".to_string()
    });

    let execution_max_iterations = env_with_default("EXECUTION_MAX_ITERATIONS", "2");
    let timeout_ms = env_with_default("TIMEOUT_MS", "30000");
    let executor_agent_max_iterations = env_with_default("EXECUTOR_AGENT_MAX_ITERATIONS", "20");
    let validation_from_planning_context = env_bool("ORCH_VALIDATION_FROM_PLANNING_CONTEXT", false);
    let reviewer_strict = env_bool("REVIEWER_STRICT", true);
    let reviewer_validation_commands = env_with_default(
        "REVIEWER_VALIDATION_COMMANDS",
        "cargo check -p autonomy_orchestrator_ai",
    );
    let delivery_enabled = env_bool("DELIVERY_ENABLED", false);
    let delivery_dry_run = env_bool("DELIVERY_DRY_RUN", true);
    let delivery_branch = env::var("DELIVERY_BRANCH").ok();
    let delivery_commit_message = env::var("DELIVERY_COMMIT_MESSAGE").ok();
    let delivery_pr_enabled = env_bool("DELIVERY_PR_ENABLED", false);
    let delivery_pr_base = env::var("DELIVERY_PR_BASE").ok();
    let delivery_pr_title = env::var("DELIVERY_PR_TITLE").ok();
    let delivery_pr_body = env::var("DELIVERY_PR_BODY").ok();

    let manager_out_dir = out_dir.join("manager");
    let executor_out_dir = out_dir.join("executor");
    let reviewer_out_dir = out_dir.join("reviewer");
    let planning_context_artifact = out_dir.join("planning/repo_context.json");
    let executor_config_base = executor_out_dir.join("agent_config");
    let executor_audit_log = executor_out_dir.join("audit.log");
    let executor_run_report = executor_out_dir.join("agent_run_report.json");
    let executor_run_replay_json = executor_out_dir.join("agent_run_replay.json");
    let executor_run_replay_text = executor_out_dir.join("agent_run_replay.txt");
    let executor_checkpoint = executor_out_dir.join("agent_checkpoint.json");
    let reviewer_report = reviewer_out_dir.join("review_report.json");

    create_dir(&out_dir)?;
    create_dir(&manager_out_dir)?;
    create_dir(&executor_out_dir)?;
    create_dir(&reviewer_out_dir)?;
    if let Some(parent) = planning_context_artifact.parent() {
        create_dir(parent)?;
    }

    fs::write(
        format!("{}.ron", executor_config_base.display()),
        format!("(\n  max_iterations: {executor_agent_max_iterations},\n)\n"),
    )
    .map_err(|error| format!("Failed to write executor config: {error}"))?;

    println!(
        "[orchestrator] Building binaries (autonomy_orchestrator_ai, auto_manager_ai, autonomous_dev_ai, autonomy_reviewer_ai)..."
    );
    run_command(
        Command::new("cargo")
            .arg("build")
            .arg("-p")
            .arg("autonomy_orchestrator_ai")
            .arg("-p")
            .arg("auto_manager_ai")
            .arg("-p")
            .arg("autonomous_dev_ai")
            .arg("-p")
            .arg("autonomy_reviewer_ai")
            .current_dir(&root_dir),
        "cargo build",
    )?;

    println!("[orchestrator] Running linked AI stack");
    println!("  out_dir={}", out_dir.display());
    println!("  repo_root={}", repo_root.display());
    println!("  goal={goal}");
    println!("  execution_max_iterations={execution_max_iterations}");
    println!("  executor_agent_max_iterations={executor_agent_max_iterations}");
    println!("  validation_from_planning_context={validation_from_planning_context}");
    println!("  reviewer_strict={reviewer_strict}");
    println!("  delivery_enabled={delivery_enabled}");
    println!("  delivery_dry_run={delivery_dry_run}");
    println!("  timeout_ms={timeout_ms}");
    println!();

    let manager_bin = root_dir.join("target/debug/auto_manager_ai");
    let executor_bin = root_dir.join("target/debug/autonomous_dev_ai");
    let reviewer_bin = root_dir.join("target/debug/autonomy_reviewer_ai");
    let orchestrator_bin = root_dir.join("target/debug/autonomy_orchestrator_ai");

    let mut orchestrator_args: Vec<String> = vec![
        out_dir.display().to_string(),
        "--repo-root".to_string(),
        repo_root.display().to_string(),
        "--planning-context-artifact".to_string(),
        planning_context_artifact.display().to_string(),
        "--policy-status".to_string(),
        "allow".to_string(),
        "--ci-status".to_string(),
        "success".to_string(),
        "--review-status".to_string(),
        "approved".to_string(),
        "--timeout-ms".to_string(),
        timeout_ms,
        "--execution-max-iterations".to_string(),
        execution_max_iterations,
        "--manager-bin".to_string(),
        manager_bin.display().to_string(),
        "--manager-env".to_string(),
        "AUTO_MANAGER_ENGINE_AVAILABLE=true".to_string(),
        "--manager-env".to_string(),
        "AUTO_MANAGER_RUN_MODE=deterministic_fallback".to_string(),
        "--manager-arg".to_string(),
        repo_root.display().to_string(),
        "--manager-arg".to_string(),
        manager_out_dir.display().to_string(),
        "--manager-expected-artifact".to_string(),
        manager_out_dir
            .join("action_plan.json")
            .display()
            .to_string(),
        "--manager-expected-artifact".to_string(),
        manager_out_dir
            .join("run_report.json")
            .display()
            .to_string(),
        "--executor-bin".to_string(),
        executor_bin.display().to_string(),
        "--executor-env".to_string(),
        format!("AUTONOMOUS_REPO_ROOT={}", repo_root.display()),
        "--executor-env".to_string(),
        format!(
            "AUTONOMOUS_RUN_REPORT_PATH={}",
            executor_run_report.display()
        ),
        "--executor-env".to_string(),
        format!(
            "AUTONOMOUS_RUN_REPLAY_PATH={}",
            executor_run_replay_json.display()
        ),
        "--executor-env".to_string(),
        format!(
            "AUTONOMOUS_RUN_REPLAY_TEXT_PATH={}",
            executor_run_replay_text.display()
        ),
        "--executor-env".to_string(),
        format!(
            "AUTONOMOUS_CHECKPOINT_PATH={}",
            executor_checkpoint.display()
        ),
        "--executor-env".to_string(),
        "AUTONOMOUS_ASSUME_VALIDATION_PASS_WHEN_NO_TEST_STEP=true".to_string(),
        "--executor-arg".to_string(),
        "--symbolic-only".to_string(),
        "--executor-arg".to_string(),
        goal,
        "--executor-arg".to_string(),
        executor_config_base.display().to_string(),
        "--executor-arg".to_string(),
        executor_audit_log.display().to_string(),
        "--executor-expected-artifact".to_string(),
        executor_audit_log.display().to_string(),
        "--executor-expected-artifact".to_string(),
        executor_run_report.display().to_string(),
        "--executor-expected-artifact".to_string(),
        executor_run_replay_json.display().to_string(),
        "--executor-expected-artifact".to_string(),
        executor_run_replay_text.display().to_string(),
        "--reviewer-bin".to_string(),
        reviewer_bin.display().to_string(),
        "--reviewer-arg".to_string(),
        repo_root.display().to_string(),
        "--reviewer-arg".to_string(),
        reviewer_out_dir.display().to_string(),
        "--reviewer-arg".to_string(),
        "--manager-action-plan".to_string(),
        "--reviewer-arg".to_string(),
        manager_out_dir
            .join("action_plan.json")
            .display()
            .to_string(),
        "--reviewer-arg".to_string(),
        "--manager-run-report".to_string(),
        "--reviewer-arg".to_string(),
        manager_out_dir
            .join("run_report.json")
            .display()
            .to_string(),
        "--reviewer-arg".to_string(),
        "--executor-run-report".to_string(),
        "--reviewer-arg".to_string(),
        executor_run_report.display().to_string(),
        "--reviewer-arg".to_string(),
        "--executor-audit-log".to_string(),
        "--reviewer-arg".to_string(),
        executor_audit_log.display().to_string(),
        "--reviewer-expected-artifact".to_string(),
        reviewer_report.display().to_string(),
    ];

    if reviewer_strict {
        orchestrator_args.push("--reviewer-arg".to_string());
        orchestrator_args.push("--strict".to_string());
    }

    for command in reviewer_validation_commands.split(";;") {
        let trimmed = command.trim();
        if !trimmed.is_empty() {
            let tokens = trimmed
                .split_whitespace()
                .map(ToString::to_string)
                .collect::<Vec<_>>();
            let Some((bin, args)) = tokens.split_first() else {
                continue;
            };
            orchestrator_args.push("--reviewer-arg".to_string());
            orchestrator_args.push("--validation-bin".to_string());
            orchestrator_args.push("--reviewer-arg".to_string());
            orchestrator_args.push(bin.clone());
            for arg in args {
                orchestrator_args.push("--reviewer-arg".to_string());
                orchestrator_args.push("--validation-arg".to_string());
                orchestrator_args.push("--reviewer-arg".to_string());
                orchestrator_args.push(arg.clone());
            }
        }
    }

    if validation_from_planning_context {
        orchestrator_args.push("--validation-from-planning-context".to_string());
    }

    if delivery_enabled {
        orchestrator_args.push("--delivery-enabled".to_string());
    }
    if delivery_dry_run {
        orchestrator_args.push("--delivery-dry-run".to_string());
    }
    if let Some(branch) = delivery_branch {
        orchestrator_args.push("--delivery-branch".to_string());
        orchestrator_args.push(branch);
    }
    if let Some(message) = delivery_commit_message {
        orchestrator_args.push("--delivery-commit-message".to_string());
        orchestrator_args.push(message);
    }
    if delivery_pr_enabled {
        orchestrator_args.push("--delivery-pr-enabled".to_string());
    }
    if let Some(base) = delivery_pr_base {
        orchestrator_args.push("--delivery-pr-base".to_string());
        orchestrator_args.push(base);
    }
    if let Some(title) = delivery_pr_title {
        orchestrator_args.push("--delivery-pr-title".to_string());
        orchestrator_args.push(title);
    }
    if let Some(body) = delivery_pr_body {
        orchestrator_args.push("--delivery-pr-body".to_string());
        orchestrator_args.push(body);
    }

    let status = Command::new(orchestrator_bin)
        .args(&orchestrator_args)
        .current_dir(&root_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|error| format!("Failed to run autonomy_orchestrator_ai: {error}"))?;
    let code = status.code().unwrap_or(1);

    println!();
    println!("[orchestrator] Completed (exit_code={code}).");
    println!(
        "  run_report={}",
        out_dir.join("orchestrator_run_report.json").display()
    );
    println!(
        "  checkpoint={}",
        out_dir.join("orchestrator_checkpoint.json").display()
    );
    println!("  planning_context={}", planning_context_artifact.display());
    println!(
        "  manager_action_plan={}",
        manager_out_dir.join("action_plan.json").display()
    );
    println!(
        "  manager_run_report={}",
        manager_out_dir.join("run_report.json").display()
    );
    println!("  executor_audit_log={}", executor_audit_log.display());
    println!("  executor_run_report={}", executor_run_report.display());
    println!(
        "  executor_run_replay_json={}",
        executor_run_replay_json.display()
    );
    println!(
        "  executor_run_replay_text={}",
        executor_run_replay_text.display()
    );
    println!("  executor_checkpoint={}", executor_checkpoint.display());
    println!("  reviewer_report={}", reviewer_report.display());

    if status.success() {
        Ok(())
    } else {
        Err(format!("Linked AI stack failed with exit code {code}"))
    }
}

fn run_command(command: &mut Command, label: &str) -> Result<(), String> {
    let status = command
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|error| format!("Failed to run {label}: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "{label} failed with exit code {}",
            status.code().unwrap_or(1)
        ))
    }
}

fn git_toplevel() -> Result<PathBuf, String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .stdin(Stdio::null())
        .output()
        .map_err(|error| format!("Failed to run git rev-parse --show-toplevel: {error}"))?;

    if !output.status.success() {
        return Err("Unable to resolve repository root via git.".to_string());
    }

    let text = String::from_utf8(output.stdout)
        .map_err(|error| format!("Invalid utf8 from git rev-parse output: {error}"))?;
    let root = text.trim();
    if root.is_empty() {
        return Err("Repository root is empty.".to_string());
    }
    Ok(Path::new(root).to_path_buf())
}

fn create_dir(path: &Path) -> Result<(), String> {
    fs::create_dir_all(path).map_err(|error| {
        format!(
            "Failed to create directory '{}': {error}",
            path.to_string_lossy()
        )
    })
}

fn env_with_default(name: &str, default: &str) -> String {
    env::var(name).unwrap_or_else(|_| default.to_string())
}

fn env_bool(name: &str, default: bool) -> bool {
    match env::var(name) {
        Ok(value) => matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        ),
        Err(_) => default,
    }
}

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  autonomy_orchestrator_ai linked-stack [out_dir] [repo_root] [goal]");
    eprintln!();
    eprintln!("Env knobs:");
    eprintln!("  EXECUTION_MAX_ITERATIONS (default: 2)");
    eprintln!("  TIMEOUT_MS (default: 30000)");
    eprintln!("  EXECUTOR_AGENT_MAX_ITERATIONS (default: 20)");
    eprintln!("  ORCH_VALIDATION_FROM_PLANNING_CONTEXT (default: false)");
    eprintln!("  REVIEWER_STRICT (default: true)");
    eprintln!("  DELIVERY_ENABLED (default: false)");
    eprintln!("  DELIVERY_DRY_RUN (default: true)");
    eprintln!("  DELIVERY_BRANCH");
    eprintln!("  DELIVERY_COMMIT_MESSAGE");
    eprintln!("  DELIVERY_PR_ENABLED (default: false)");
    eprintln!("  DELIVERY_PR_BASE");
    eprintln!("  DELIVERY_PR_TITLE");
    eprintln!("  DELIVERY_PR_BODY");
    eprintln!(
        "  REVIEWER_VALIDATION_COMMANDS (default: 'cargo check -p autonomy_orchestrator_ai', separator: ';;')"
    );
}
