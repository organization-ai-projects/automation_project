use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_temp_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let pid = std::process::id();
    let dir = std::env::temp_dir().join(format!("autonomous_dev_ai_{name}_{pid}_{nanos}"));
    fs::create_dir_all(&dir).expect("failed to create temp dir");
    dir
}

fn fixture_repo_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join("ci_like_repo")
}

fn write_test_config(config_ron_path: &Path) {
    let content = r#"(
    agent_name: "autonomous_dev_ai",
    execution_mode: "ci_bound",
    neural: (
        enabled: true,
        prefer_gpu: false,
        cpu_fallback: true,
        models: {
            "intent": "intent_v1.bin",
            "codegen": "codegen_v2.bin",
            "review": "review_v1.bin",
        },
    ),
    symbolic: (
        strict_validation: true,
        deterministic: true,
    ),
    objectives: [
        (
            name: "task_completion",
            weight: 1.0,
            hard: true,
            threshold: Some(0.5),
        ),
        (
            name: "policy_safety",
            weight: 1.0,
            hard: true,
            threshold: Some(0.0),
        ),
        (
            name: "tests_pass",
            weight: 0.9,
            hard: true,
            threshold: Some(0.7),
        ),
        (
            name: "minimal_diff",
            weight: 0.6,
            hard: false,
            threshold: None,
        ),
        (
            name: "time_budget",
            weight: 0.4,
            hard: false,
            threshold: None,
        ),
    ],
    max_iterations: 8,
    timeout_seconds: Some(600),
)
"#;
    fs::write(config_ron_path, content).expect("failed to write test config");
}

fn run_binary(
    goal: &str,
    symbolic_only: bool,
    extra_env: &[(&str, &str)],
) -> (Output, PathBuf, PathBuf) {
    let bin = env!("CARGO_BIN_EXE_autonomous_dev_ai");
    let fixture = fixture_repo_dir();
    let out_dir = unique_temp_dir("resilience");

    let config_base = out_dir.join("agent_config");
    let config_ron = config_base.with_extension("ron");
    write_test_config(&config_ron);

    let audit_log = out_dir.join("agent_audit.log");
    let run_report = out_dir.join("agent_run_report.json");
    let run_replay = out_dir.join("agent_run_replay.json");
    let run_replay_txt = out_dir.join("agent_run_replay.txt");
    let checkpoint = out_dir.join("agent_checkpoint.json");
    let pr_description = out_dir.join("pr_description.md");

    let mut cmd = Command::new(bin);
    cmd.current_dir(&fixture);
    if symbolic_only {
        cmd.arg("--symbolic-only");
    }
    cmd.arg(goal)
        .arg(&config_base)
        .arg(&audit_log)
        .env("AUTONOMOUS_RUN_REPORT_PATH", &run_report)
        .env("AUTONOMOUS_RUN_REPLAY_PATH", &run_replay)
        .env("AUTONOMOUS_RUN_REPLAY_TEXT_PATH", &run_replay_txt)
        .env("AUTONOMOUS_CHECKPOINT_PATH", &checkpoint)
        .env("AUTONOMOUS_REPO_ROOT", &fixture)
        .env("AUTONOMOUS_REQUIRE_EXPLORED_FILES", "true")
        .env(
            "AUTONOMOUS_ISSUE_TITLE",
            "feat(autonomous_dev_ai): resilience test",
        )
        .env(
            "AUTONOMOUS_ISSUE_BODY",
            "Context\nResilience integration test\n\nHierarchy\nParent: none",
        )
        .env("AUTONOMOUS_REVIEW_REQUIRED", "true")
        .env(
            "AUTONOMOUS_REVIEW_COMMENTS_JSON",
            "[{\"reviewer\":\"ci-bot\",\"body\":\"approved\",\"resolved\":true}]",
        )
        .env("AUTONOMOUS_PR_NUMBER", "649")
        .env("AUTONOMOUS_PR_DESCRIPTION_OUTPUT", &pr_description);

    for (k, v) in extra_env {
        cmd.env(k, v);
    }

    let output = cmd
        .output()
        .expect("failed to execute autonomous_dev_ai binary");

    (output, run_report, out_dir)
}

fn spawn_binary_with_lock(
    goal: &str,
    lock_path: &Path,
    hold_lock_ms: u64,
) -> (Child, PathBuf, PathBuf) {
    let bin = env!("CARGO_BIN_EXE_autonomous_dev_ai");
    let fixture = fixture_repo_dir();
    let out_dir = unique_temp_dir("lock_holder");

    let config_base = out_dir.join("agent_config");
    let config_ron = config_base.with_extension("ron");
    write_test_config(&config_ron);

    let audit_log = out_dir.join("agent_audit.log");
    let run_report = out_dir.join("agent_run_report.json");
    let run_replay = out_dir.join("agent_run_replay.json");
    let run_replay_txt = out_dir.join("agent_run_replay.txt");
    let checkpoint = out_dir.join("agent_checkpoint.json");
    let pr_description = out_dir.join("pr_description.md");

    let child = Command::new(bin)
        .current_dir(&fixture)
        .arg(goal)
        .arg(&config_base)
        .arg(&audit_log)
        .env("AUTONOMOUS_RUN_REPORT_PATH", &run_report)
        .env("AUTONOMOUS_RUN_REPLAY_PATH", &run_replay)
        .env("AUTONOMOUS_RUN_REPLAY_TEXT_PATH", &run_replay_txt)
        .env("AUTONOMOUS_CHECKPOINT_PATH", &checkpoint)
        .env("AUTONOMOUS_RUNTIME_LOCK_PATH", lock_path)
        .env("AUTONOMOUS_HOLD_LOCK_MS", hold_lock_ms.to_string())
        .env("AUTONOMOUS_REPO_ROOT", &fixture)
        .env("AUTONOMOUS_REQUIRE_EXPLORED_FILES", "true")
        .env(
            "AUTONOMOUS_ISSUE_TITLE",
            "feat(autonomous_dev_ai): runtime lock holder",
        )
        .env(
            "AUTONOMOUS_ISSUE_BODY",
            "Context\nResilience lock holder\n\nHierarchy\nParent: none",
        )
        .env("AUTONOMOUS_REVIEW_REQUIRED", "true")
        .env(
            "AUTONOMOUS_REVIEW_COMMENTS_JSON",
            "[{\"reviewer\":\"ci-bot\",\"body\":\"approved\",\"resolved\":true}]",
        )
        .env("AUTONOMOUS_PR_NUMBER", "649")
        .env("AUTONOMOUS_PR_DESCRIPTION_OUTPUT", &pr_description)
        .spawn()
        .expect("failed to spawn lock holder binary");

    (child, run_report, out_dir)
}

#[test]
fn symbolic_only_mode_is_deterministic_for_core_fields() {
    let (first_output, first_report_path, first_dir) = run_binary(
        "Validate deterministic symbolic-only flow for issue #649 with tests",
        true,
        &[],
    );
    let (second_output, second_report_path, second_dir) = run_binary(
        "Validate deterministic symbolic-only flow for issue #649 with tests",
        true,
        &[],
    );

    assert!(
        first_output.status.success(),
        "first symbolic-only run failed"
    );
    assert!(
        second_output.status.success(),
        "second symbolic-only run failed"
    );

    let first_raw = fs::read_to_string(&first_report_path).expect("missing first run report");
    let second_raw = fs::read_to_string(&second_report_path).expect("missing second run report");
    let first: serde_json::Value = serde_json::from_str(&first_raw).expect("first report invalid");
    let second: serde_json::Value =
        serde_json::from_str(&second_raw).expect("second report invalid");

    assert_eq!(first["final_state"], "Done");
    assert_eq!(second["final_state"], "Done");
    assert_eq!(first["neural_enabled"], false);
    assert_eq!(second["neural_enabled"], false);

    assert_eq!(first["total_iterations"], second["total_iterations"]);
    assert_eq!(first["total_decisions"], second["total_decisions"]);
    assert_eq!(first["total_failures"], second["total_failures"]);
    assert_eq!(
        first["last_objective_passed"],
        second["last_objective_passed"]
    );

    let _ = fs::remove_dir_all(first_dir);
    let _ = fs::remove_dir_all(second_dir);
}

#[test]
fn retry_recovery_path_retries_before_terminal_failure() {
    let missing_cmd = "definitely_missing_test_cmd_xyz";
    let (output, run_report_path, out_dir) = run_binary(
        "Validate retry and recovery behavior for tests",
        false,
        &[
            ("AUTONOMOUS_TEST_COMMAND", missing_cmd),
            ("AUTONOMOUS_ALLOWED_TEST_PROGRAMS", missing_cmd),
        ],
    );

    assert!(
        !output.status.success(),
        "binary unexpectedly succeeded. stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let combined = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        combined.contains("Recoverable error (attempt 1/3)"),
        "missing retry trace in output:\n{}",
        combined
    );
    assert!(
        combined.contains("Recoverable error exhausted retries"),
        "missing retry exhaustion trace in output:\n{}",
        combined
    );

    let raw = fs::read_to_string(&run_report_path).expect("missing run report");
    let json: serde_json::Value = serde_json::from_str(&raw).expect("run report invalid");
    assert_ne!(json["final_state"], "Done");
    assert_eq!(json["last_tool_name"], "run_tests");

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn policy_deny_path_blocks_execution_with_risk_gate() {
    let (output, run_report_path, out_dir) = run_binary(
        "Validate policy deny path for high-risk test action",
        false,
        &[
            ("AUTONOMOUS_TOOL_RISK_OVERRIDES", "run_tests=high"),
            ("AUTONOMOUS_POLICY_PACK_AUTO_SIGN", "true"),
        ],
    );

    assert!(
        !output.status.success(),
        "binary unexpectedly succeeded. stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let combined = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        combined.contains("requires approval token") || combined.contains("risk gate"),
        "missing policy deny signal in output:\n{}",
        combined
    );

    let raw = fs::read_to_string(&run_report_path).expect("missing run report");
    let json: serde_json::Value = serde_json::from_str(&raw).expect("run report invalid");
    assert_ne!(json["final_state"], "Done");
    assert!(json["risk_gate_denies"].as_u64().unwrap_or(0) > 0);

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn forbidden_force_push_pattern_is_blocked_by_policy_validation() {
    let (output, run_report_path, out_dir) = run_binary(
        "Validate test command safety for force push operation",
        false,
        &[
            ("AUTONOMOUS_TEST_COMMAND", "git push --force"),
            ("AUTONOMOUS_ALLOWED_TEST_PROGRAMS", "git"),
        ],
    );

    assert!(
        !output.status.success(),
        "binary unexpectedly succeeded. stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let combined = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        combined.contains("violates policy patterns") || combined.contains("Policy violation"),
        "missing policy violation signal in output:\n{}",
        combined
    );
    assert!(
        !combined.contains("Recoverable error (attempt"),
        "force-push policy violation should be fatal (no retries):\n{}",
        combined
    );

    let raw = fs::read_to_string(&run_report_path).expect("missing run report");
    let json: serde_json::Value = serde_json::from_str(&raw).expect("run report invalid");
    assert_ne!(json["final_state"], "Done");

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn concurrent_run_is_rejected_by_runtime_lock() {
    let shared_lock = unique_temp_dir("shared_lock").join("autonomous.runtime.lock");
    let (mut holder, _holder_report, holder_dir) = spawn_binary_with_lock(
        "Hold runtime lock for resilience contention test",
        &shared_lock,
        4000,
    );

    std::thread::sleep(std::time::Duration::from_millis(500));
    let lock_path_value = shared_lock.to_string_lossy().to_string();
    let (output, _run_report_path, out_dir) = run_binary(
        "Second run should fail on runtime lock contention",
        false,
        &[("AUTONOMOUS_RUNTIME_LOCK_PATH", &lock_path_value)],
    );

    assert!(
        !output.status.success(),
        "second run unexpectedly succeeded under lock contention. stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let combined = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        combined.contains("Failed to acquire runtime lock")
            || combined.contains("runtime lock is already held"),
        "missing runtime-lock error in output:\n{}",
        combined
    );

    let _ = holder.kill();
    let _ = holder.wait();
    let _ = fs::remove_dir_all(out_dir);
    let _ = fs::remove_dir_all(holder_dir);
}

#[test]
fn cpu_budget_guard_triggers_fail_safe_state() {
    let (output, run_report_path, out_dir) = run_binary(
        "Validate cpu budget enforcement",
        false,
        &[("AUTONOMOUS_MAX_CPU_SECONDS", "0")],
    );

    assert!(
        !output.status.success(),
        "binary unexpectedly succeeded with zero cpu budget. stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let raw = fs::read_to_string(&run_report_path).expect("missing run report");
    let json: serde_json::Value = serde_json::from_str(&raw).expect("run report invalid");
    assert_ne!(json["final_state"], "Done");
    assert!(
        json["total_failures"].as_u64().unwrap_or(0) >= 1,
        "expected at least one failure in run report: {}",
        raw
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn checkpoint_resume_recovers_after_failed_run() {
    let bin = env!("CARGO_BIN_EXE_autonomous_dev_ai");
    let fixture = fixture_repo_dir();
    let out_dir = unique_temp_dir("checkpoint_resume");

    let config_base = out_dir.join("agent_config");
    let config_ron = config_base.with_extension("ron");
    write_test_config(&config_ron);

    let audit_log = out_dir.join("agent_audit.log");
    let run_report = out_dir.join("agent_run_report.json");
    let run_replay = out_dir.join("agent_run_replay.json");
    let run_replay_txt = out_dir.join("agent_run_replay.txt");
    let checkpoint = out_dir.join("agent_checkpoint.json");
    let pr_description = out_dir.join("pr_description.md");

    let first = Command::new(bin)
        .current_dir(&fixture)
        .arg("Force a first failure to create checkpoint")
        .arg(&config_base)
        .arg(&audit_log)
        .env("AUTONOMOUS_RUN_REPORT_PATH", &run_report)
        .env("AUTONOMOUS_RUN_REPLAY_PATH", &run_replay)
        .env("AUTONOMOUS_RUN_REPLAY_TEXT_PATH", &run_replay_txt)
        .env("AUTONOMOUS_CHECKPOINT_PATH", &checkpoint)
        .env("AUTONOMOUS_REPO_ROOT", &fixture)
        .env("AUTONOMOUS_REQUIRE_EXPLORED_FILES", "true")
        .env(
            "AUTONOMOUS_ISSUE_TITLE",
            "feat(autonomous_dev_ai): checkpoint resume",
        )
        .env(
            "AUTONOMOUS_ISSUE_BODY",
            "Context\nCheckpoint resume test\n\nHierarchy\nParent: none",
        )
        .env("AUTONOMOUS_REVIEW_REQUIRED", "true")
        .env(
            "AUTONOMOUS_REVIEW_COMMENTS_JSON",
            "[{\"reviewer\":\"ci-bot\",\"body\":\"approved\",\"resolved\":true}]",
        )
        .env("AUTONOMOUS_PR_NUMBER", "649")
        .env("AUTONOMOUS_PR_DESCRIPTION_OUTPUT", &pr_description)
        .env("AUTONOMOUS_TEST_COMMAND", "definitely_missing_test_cmd_xyz")
        .env(
            "AUTONOMOUS_ALLOWED_TEST_PROGRAMS",
            "definitely_missing_test_cmd_xyz",
        )
        .output()
        .expect("failed to execute first run");

    assert!(
        !first.status.success(),
        "first run unexpectedly succeeded. stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&first.stdout),
        String::from_utf8_lossy(&first.stderr)
    );
    assert!(
        checkpoint.exists(),
        "checkpoint file was not created at {}",
        checkpoint.display()
    );

    let second = Command::new(bin)
        .current_dir(&fixture)
        .arg("--resume")
        .arg("Resume from checkpoint and complete")
        .arg(&config_base)
        .arg(&audit_log)
        .env("AUTONOMOUS_RUN_REPORT_PATH", &run_report)
        .env("AUTONOMOUS_RUN_REPLAY_PATH", &run_replay)
        .env("AUTONOMOUS_RUN_REPLAY_TEXT_PATH", &run_replay_txt)
        .env("AUTONOMOUS_CHECKPOINT_PATH", &checkpoint)
        .env("AUTONOMOUS_REPO_ROOT", &fixture)
        .env("AUTONOMOUS_REQUIRE_EXPLORED_FILES", "true")
        .env(
            "AUTONOMOUS_ISSUE_TITLE",
            "feat(autonomous_dev_ai): checkpoint resume",
        )
        .env(
            "AUTONOMOUS_ISSUE_BODY",
            "Context\nCheckpoint resume test\n\nHierarchy\nParent: none",
        )
        .env("AUTONOMOUS_REVIEW_REQUIRED", "true")
        .env(
            "AUTONOMOUS_REVIEW_COMMENTS_JSON",
            "[{\"reviewer\":\"ci-bot\",\"body\":\"approved\",\"resolved\":true}]",
        )
        .env("AUTONOMOUS_PR_NUMBER", "649")
        .env("AUTONOMOUS_PR_DESCRIPTION_OUTPUT", &pr_description)
        .output()
        .expect("failed to execute resumed run");

    assert!(
        second.status.success(),
        "resumed run failed. stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&second.stdout),
        String::from_utf8_lossy(&second.stderr)
    );

    let replay_raw = fs::read_to_string(&run_replay).expect("missing run replay");
    assert!(
        replay_raw.contains("checkpoint.loaded"),
        "checkpoint resume event missing in run replay: {}",
        replay_raw
    );

    let _ = fs::remove_dir_all(out_dir);
}
