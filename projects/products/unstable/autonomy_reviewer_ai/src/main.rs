use common_json::{Json, JsonAccess, from_str, to_string_pretty};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{self, Command};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize)]
struct ReviewReport {
    product: String,
    version: String,
    run_id: String,
    strict_mode: bool,
    passed: bool,
    findings: Vec<ReviewItem>,
    warnings: Vec<ReviewItem>,
    next_step_plan: Vec<NextStep>,
    checked_artifacts: Vec<String>,
    executed_validation_commands: Vec<String>,
    failed_validation_commands: Vec<String>,
    generated_at_unix_secs: u64,
}

#[derive(Debug, Serialize)]
struct ReviewItem {
    code: String,
    message: String,
    remediation: String,
}

#[derive(Debug, Serialize)]
struct NextStep {
    priority: u32,
    source: String,
    code: String,
    action: String,
}

#[derive(Debug, Default)]
struct ReviewInputs {
    repo_root: PathBuf,
    output_dir: PathBuf,
    manager_action_plan: Option<PathBuf>,
    manager_run_report: Option<PathBuf>,
    executor_run_report: Option<PathBuf>,
    executor_audit_log: Option<PathBuf>,
    validation_commands: Vec<String>,
    validation_shell: String,
    strict: bool,
}

fn main() {
    let inputs = parse_args();

    println!("Autonomy Reviewer AI");
    println!("Repo root: {}", inputs.repo_root.display());
    println!("Output: {}", inputs.output_dir.display());
    println!("Strict mode: {}", inputs.strict);
    println!();

    let mut findings = Vec::new();
    let mut warnings = Vec::new();
    let mut checked_artifacts = Vec::new();
    let mut executed_validation_commands = Vec::new();
    let mut failed_validation_commands = Vec::new();

    check_json_artifact(
        "manager_action_plan",
        &inputs.manager_action_plan,
        &mut findings,
        &mut warnings,
        &mut checked_artifacts,
        |json, warns, _| {
            ensure_string_field("schema_version", json, warns);
            ensure_string_field("producer", json, warns);
            ensure_string_field("summary", json, warns);
        },
    );

    check_json_artifact(
        "manager_run_report",
        &inputs.manager_run_report,
        &mut findings,
        &mut warnings,
        &mut checked_artifacts,
        |json, warns, _| {
            ensure_string_field("schema_version", json, warns);
            ensure_string_field("producer", json, warns);
            ensure_string_field("status", json, warns);
        },
    );

    check_json_artifact(
        "executor_run_report",
        &inputs.executor_run_report,
        &mut findings,
        &mut warnings,
        &mut checked_artifacts,
        |json, warns, finds| {
            ensure_string_field("run_id", json, warns);
            ensure_string_field("final_state", json, warns);
            ensure_bool_field("closure_gates_satisfied", json, warns);
            if let Ok(final_state) = json
                .get_field("final_state")
                .and_then(|v| v.as_str_strict())
                && final_state != "Completed"
                && final_state != "Done"
            {
                finds.push(item(
                    "EXECUTOR_FINAL_STATE_NOT_COMPLETED",
                    format!(
                        "executor_run_report final_state is '{}' (expected 'Completed' or 'Done')",
                        final_state
                    ),
                    "Inspect executor run report/audit log, fix root cause, and rerun execution stage.",
                ));
            }
            if let Ok(closure_ok) = json
                .get_field("closure_gates_satisfied")
                .and_then(|v| v.as_bool_strict())
                && !closure_ok
            {
                let non_interactive_profile = json
                    .get_field("non_interactive_profile")
                    .and_then(|v| v.as_str_strict())
                    .ok();
                let create_pr_enabled = json
                    .get_field("create_pr_enabled")
                    .and_then(|v| v.as_bool_strict())
                    .unwrap_or(false);
                let review_required = json
                    .get_field("review_required")
                    .and_then(|v| v.as_bool_strict())
                    .unwrap_or(false);
                let requires_strict_closure = non_interactive_profile == Some("orchestrator_v1")
                    || create_pr_enabled
                    || review_required;

                if requires_strict_closure {
                    finds.push(item(
                        "EXECUTOR_CLOSURE_GATES_NOT_SATISFIED",
                        "executor_run_report closure_gates_satisfied=false (expected true)"
                            .to_string(),
                        "Review unmet closure gates in executor report and satisfy required PR/CI/review constraints.",
                    ));
                }
            }
            if let Ok(failures) = json
                .get_field("total_failures")
                .and_then(|v| v.as_u64_strict())
                && failures > 5
            {
                warns.push(item(
                    "EXECUTOR_TOTAL_FAILURES_NON_ZERO",
                    format!(
                        "executor_run_report total_failures={} (review warning, threshold > 5)",
                        failures
                    ),
                    "Reduce executor failure count by fixing recurring tool/policy failures before merge.",
                ));
            }
        },
    );

    if let Some(path) = &inputs.executor_audit_log {
        checked_artifacts.push(path.display().to_string());
        match fs::read_to_string(path) {
            Ok(content) => {
                if content.trim().is_empty() {
                    warnings.push(item(
                        "EXECUTOR_AUDIT_LOG_EMPTY",
                        format!("executor_audit_log is empty: '{}'", path.display()),
                        "Ensure executor writes meaningful audit events for traceability.",
                    ));
                }
            }
            Err(err) => findings.push(item(
                "EXECUTOR_AUDIT_LOG_MISSING",
                format!(
                    "executor_audit_log is missing/unreadable '{}': {}",
                    path.display(),
                    err
                ),
                "Ensure the executor is configured with a writable audit log path and rerun.",
            )),
        }
    } else {
        warnings.push(item(
            "EXECUTOR_AUDIT_LOG_NOT_PROVIDED",
            "executor_audit_log path not provided".to_string(),
            "Provide --executor-audit-log in reviewer invocation for traceability.",
        ));
    }

    for command_text in &inputs.validation_commands {
        executed_validation_commands.push(command_text.clone());
        let status = Command::new(&inputs.validation_shell)
            .arg("-c")
            .arg(command_text)
            .current_dir(&inputs.repo_root)
            .status();
        match status {
            Ok(exit) if exit.success() => {}
            Ok(exit) => {
                failed_validation_commands.push(command_text.clone());
                findings.push(item(
                    "VALIDATION_COMMAND_FAILED",
                    format!(
                        "validation command failed (exit={:?}): {}",
                        exit.code(),
                        command_text
                    ),
                    "Run the command locally, fix the failing checks, then rerun reviewer/orchestrator.",
                ));
            }
            Err(err) => {
                failed_validation_commands.push(command_text.clone());
                findings.push(item(
                    "VALIDATION_COMMAND_SPAWN_FAILED",
                    format!(
                        "validation command spawn failed (shell='{}'): {} ({})",
                        inputs.validation_shell, command_text, err
                    ),
                    "Verify --validation-shell exists and command syntax is valid.",
                ));
            }
        }
    }

    let passed = findings.is_empty() && (!inputs.strict || warnings.is_empty());
    let next_step_plan = build_next_step_plan(&findings, &warnings, inputs.strict);
    let run_id = format!("review_{}", unix_timestamp_secs());
    let report = ReviewReport {
        product: "autonomy_reviewer_ai".to_string(),
        version: "0.1.0".to_string(),
        run_id,
        strict_mode: inputs.strict,
        passed,
        findings,
        warnings,
        next_step_plan,
        checked_artifacts,
        executed_validation_commands,
        failed_validation_commands,
        generated_at_unix_secs: unix_timestamp_secs(),
    };

    if let Err(err) = write_report(&inputs.output_dir, &report) {
        eprintln!("Failed to write review report: {err}");
        process::exit(1);
    }

    println!(
        "Review report: {}",
        inputs.output_dir.join("review_report.json").display()
    );
    println!("Review passed: {}", report.passed);

    if report.passed {
        process::exit(0);
    }
    process::exit(1);
}

fn parse_args() -> ReviewInputs {
    let mut args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() < 2 {
        usage_and_exit();
    }
    let repo_root = PathBuf::from(args.remove(0));
    let output_dir = PathBuf::from(args.remove(0));
    let mut manager_action_plan = None;
    let mut manager_run_report = None;
    let mut executor_run_report = None;
    let mut executor_audit_log = None;
    let mut validation_commands = Vec::new();
    let mut validation_shell = "/bin/sh".to_string();
    let mut strict = false;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--manager-action-plan" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                manager_action_plan = Some(PathBuf::from(args[i + 1].clone()));
                i += 2;
            }
            "--manager-run-report" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                manager_run_report = Some(PathBuf::from(args[i + 1].clone()));
                i += 2;
            }
            "--executor-run-report" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                executor_run_report = Some(PathBuf::from(args[i + 1].clone()));
                i += 2;
            }
            "--executor-audit-log" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                executor_audit_log = Some(PathBuf::from(args[i + 1].clone()));
                i += 2;
            }
            "--validation-command" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                validation_commands.push(args[i + 1].clone());
                i += 2;
            }
            "--validation-shell" => {
                if i + 1 >= args.len() {
                    usage_and_exit();
                }
                validation_shell = args[i + 1].clone();
                i += 2;
            }
            "--strict" => {
                strict = true;
                i += 1;
            }
            _ => usage_and_exit(),
        }
    }

    ReviewInputs {
        repo_root,
        output_dir,
        manager_action_plan,
        manager_run_report,
        executor_run_report,
        executor_audit_log,
        validation_commands,
        validation_shell,
        strict,
    }
}

fn usage_and_exit() -> ! {
    eprintln!("Usage:");
    eprintln!("  autonomy_reviewer_ai <repo_root> <output_dir> [options]");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --manager-action-plan <path>");
    eprintln!("  --manager-run-report <path>");
    eprintln!("  --executor-run-report <path>");
    eprintln!("  --executor-audit-log <path>");
    eprintln!("  --validation-command <shell command>     (repeatable)");
    eprintln!("  --validation-shell <path>                (default: /bin/sh)");
    eprintln!("  --strict");
    process::exit(2);
}

fn check_json_artifact<F: FnOnce(&Json, &mut Vec<ReviewItem>, &mut Vec<ReviewItem>)>(
    label: &str,
    path: &Option<PathBuf>,
    findings: &mut Vec<ReviewItem>,
    warnings: &mut Vec<ReviewItem>,
    checked_artifacts: &mut Vec<String>,
    semantic_checks: F,
) {
    let Some(path) = path else {
        warnings.push(item(
            &format!("{}_NOT_PROVIDED", label.to_ascii_uppercase()),
            format!("{label} path not provided"),
            "Provide this artifact path in reviewer invocation for stronger checks.",
        ));
        return;
    };
    checked_artifacts.push(path.display().to_string());
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            findings.push(item(
                &format!("{}_MISSING", label.to_ascii_uppercase()),
                format!(
                    "{label} is missing/unreadable '{}': {}",
                    path.display(),
                    err
                ),
                "Ensure upstream stage produced the artifact and that path wiring is correct.",
            ));
            return;
        }
    };
    let parsed = match from_str::<Json>(&content) {
        Ok(value) => value,
        Err(err) => {
            findings.push(item(
                &format!("{}_MALFORMED_JSON", label.to_ascii_uppercase()),
                format!("{label} JSON is malformed '{}': {:?}", path.display(), err),
                "Write valid JSON in the artifact producer and rerun.",
            ));
            return;
        }
    };
    semantic_checks(&parsed, warnings, findings);
}

fn ensure_string_field(field: &str, json: &Json, warnings: &mut Vec<ReviewItem>) {
    match json.get_field(field) {
        Ok(Json::String(_)) => {}
        _ => warnings.push(item(
            "ARTIFACT_FIELD_TYPE_MISMATCH",
            format!("expected string field '{}' in JSON artifact", field),
            "Update artifact schema or producer to emit expected string fields.",
        )),
    }
}

fn ensure_bool_field(field: &str, json: &Json, warnings: &mut Vec<ReviewItem>) {
    match json.get_field(field) {
        Ok(Json::Bool(_)) => {}
        _ => warnings.push(item(
            "ARTIFACT_FIELD_TYPE_MISMATCH",
            format!("expected boolean field '{}' in JSON artifact", field),
            "Update artifact schema or producer to emit expected boolean fields.",
        )),
    }
}

fn item(code: &str, message: String, remediation: &str) -> ReviewItem {
    ReviewItem {
        code: code.to_string(),
        message,
        remediation: remediation.to_string(),
    }
}

fn build_next_step_plan(
    findings: &[ReviewItem],
    warnings: &[ReviewItem],
    strict_mode: bool,
) -> Vec<NextStep> {
    let mut steps = Vec::new();
    let mut priority = 1u32;

    for finding in findings {
        steps.push(NextStep {
            priority,
            source: "finding".to_string(),
            code: finding.code.clone(),
            action: finding.remediation.clone(),
        });
        priority += 1;
    }

    if strict_mode || findings.is_empty() {
        for warning in warnings {
            steps.push(NextStep {
                priority,
                source: "warning".to_string(),
                code: warning.code.clone(),
                action: warning.remediation.clone(),
            });
            priority += 1;
        }
    }

    steps
}

fn write_report(output_dir: &Path, report: &ReviewReport) -> Result<(), String> {
    fs::create_dir_all(output_dir).map_err(|e| {
        format!(
            "Failed to create output dir '{}': {}",
            output_dir.display(),
            e
        )
    })?;
    let report_json = to_string_pretty(report)
        .map_err(|e| format!("Failed to serialize reviewer report: {e:?}"))?;
    let report_path = output_dir.join("review_report.json");
    fs::write(&report_path, report_json).map_err(|e| {
        format!(
            "Failed to write reviewer report '{}': {}",
            report_path.display(),
            e
        )
    })
}

fn unix_timestamp_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
