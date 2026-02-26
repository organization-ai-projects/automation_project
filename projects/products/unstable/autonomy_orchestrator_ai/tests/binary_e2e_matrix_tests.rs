use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

use common_json::from_str;

#[derive(Debug, serde::Deserialize)]
struct MatrixReportView {
    terminal_state: Option<String>,
    blocked_reason_codes: Vec<String>,
    reviewer_next_steps: Vec<String>,
    stage_executions: Vec<StageExecutionView>,
}

#[derive(Debug, serde::Deserialize)]
struct StageExecutionView {
    stage: String,
    status: String,
}

#[derive(Debug, serde::Deserialize)]
struct RepoContextView {
    repo_root: String,
    detected_validation_commands: Vec<RepoValidationInvocation>,
}

#[derive(Debug, serde::Deserialize)]
struct RepoValidationInvocation {
    command: String,
    args: Vec<String>,
}

#[derive(Debug, serde::Serialize)]
struct CheckpointFixture {
    run_id: String,
    completed_stages: Vec<String>,
    terminal_state: Option<String>,
    updated_at_unix_secs: u64,
}

fn unique_temp_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let pid = std::process::id();
    let dir =
        std::env::temp_dir().join(format!("autonomy_orchestrator_matrix_{name}_{pid}_{nanos}"));
    fs::create_dir_all(&dir).expect("failed to create temp dir");
    dir
}

fn run_orchestrator(args: &[&str], out_dir: &PathBuf) -> (Output, MatrixReportView) {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");

    let mut cmd = Command::new(bin);
    cmd.arg(out_dir);
    for arg in args {
        cmd.arg(arg);
    }

    let output = cmd.output().expect("failed to execute orchestrator binary");

    let report_path = out_dir.join("orchestrator_run_report.json");
    assert!(report_path.exists(), "run report is missing");
    let checkpoint_path = out_dir.join("orchestrator_checkpoint.json");
    assert!(checkpoint_path.exists(), "checkpoint is missing");

    let report_raw = fs::read_to_string(&report_path).expect("failed to read run report");
    let report: MatrixReportView = from_str(&report_raw).expect("failed to deserialize run report");

    (output, report)
}

fn run_orchestrator_owned(args: &[String], out_dir: &PathBuf) -> (Output, MatrixReportView) {
    let as_refs = args.iter().map(String::as_str).collect::<Vec<_>>();
    run_orchestrator(&as_refs, out_dir)
}

fn fixture_bin() -> &'static str {
    env!("CARGO_BIN_EXE_autonomy_orchestrator_ai")
}

#[test]
fn matrix_happy_path_reaches_done() {
    let out_dir = unique_temp_dir("happy_path");

    let (output, report) = run_orchestrator(
        &[
            "--policy-status",
            "allow",
            "--ci-status",
            "success",
            "--review-status",
            "approved",
            "--manager-bin",
            fixture_bin(),
            "--manager-arg",
            "fixture",
            "--manager-arg",
            "success",
            "--executor-bin",
            fixture_bin(),
            "--executor-arg",
            "fixture",
            "--executor-arg",
            "success",
            "--reviewer-bin",
            fixture_bin(),
            "--reviewer-arg",
            "fixture",
            "--reviewer-arg",
            "success",
        ],
        &out_dir,
    );

    assert!(output.status.success());
    assert_eq!(report.terminal_state.as_deref(), Some("done"));
    assert!(report.blocked_reason_codes.is_empty());
    assert_eq!(report.stage_executions.len(), 3);
    assert!(
        report
            .stage_executions
            .iter()
            .all(|e| e.status == "success")
    );
    assert!(
        report
            .stage_executions
            .iter()
            .any(|e| e.stage == "validation" && e.status == "success")
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_policy_denial_blocks_with_reason_code() {
    let out_dir = unique_temp_dir("policy_denial");

    let (output, report) = run_orchestrator(
        &[
            "--policy-status",
            "deny",
            "--ci-status",
            "success",
            "--review-status",
            "approved",
        ],
        &out_dir,
    );

    assert_eq!(output.status.code(), Some(3));
    assert_eq!(report.terminal_state.as_deref(), Some("blocked"));
    assert!(
        report
            .blocked_reason_codes
            .contains(&"GATE_POLICY_DENIED_OR_UNKNOWN".to_string())
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_missing_ci_signal_blocks_with_reason_code() {
    let out_dir = unique_temp_dir("missing_ci");

    let (output, report) = run_orchestrator(
        &[
            "--policy-status",
            "allow",
            "--ci-status",
            "missing",
            "--review-status",
            "approved",
        ],
        &out_dir,
    );

    assert_eq!(output.status.code(), Some(3));
    assert_eq!(report.terminal_state.as_deref(), Some("blocked"));
    assert!(
        report
            .blocked_reason_codes
            .contains(&"GATE_CI_NOT_SUCCESS".to_string())
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_review_rejection_blocks_with_reason_code() {
    let out_dir = unique_temp_dir("review_rejection");

    let (output, report) = run_orchestrator(
        &[
            "--policy-status",
            "allow",
            "--ci-status",
            "success",
            "--review-status",
            "changes_requested",
        ],
        &out_dir,
    );

    assert_eq!(output.status.code(), Some(3));
    assert_eq!(report.terminal_state.as_deref(), Some("blocked"));
    assert!(
        report
            .blocked_reason_codes
            .contains(&"GATE_REVIEW_NOT_APPROVED".to_string())
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_timeout_sets_timeout_terminal_state() {
    let out_dir = unique_temp_dir("timeout");

    let (output, report) = run_orchestrator(
        &[
            "--policy-status",
            "allow",
            "--ci-status",
            "success",
            "--review-status",
            "approved",
            "--timeout-ms",
            "10",
            "--manager-bin",
            fixture_bin(),
            "--manager-arg",
            "fixture",
            "--manager-arg",
            "sleep-ms",
            "--manager-arg",
            "1000",
        ],
        &out_dir,
    );

    assert_eq!(output.status.code(), Some(124));
    assert_eq!(report.terminal_state.as_deref(), Some("timeout"));
    assert!(report.blocked_reason_codes.is_empty());
    assert_eq!(report.stage_executions.len(), 1);
    assert_eq!(report.stage_executions[0].status, "timeout");

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_crash_resume_skips_completed_stage_and_finishes() {
    let out_dir = unique_temp_dir("crash_resume");
    let checkpoint_path = out_dir.join("orchestrator_checkpoint.json");

    let fixture = CheckpointFixture {
        run_id: "run_crash_resume".to_string(),
        completed_stages: vec!["planning".to_string()],
        terminal_state: None,
        updated_at_unix_secs: 1,
    };
    let fixture_json =
        common_json::to_string_pretty(&fixture).expect("serialize checkpoint fixture");
    fs::write(&checkpoint_path, fixture_json).expect("write checkpoint fixture");

    let (output, report) = run_orchestrator(
        &[
            "--resume",
            "--policy-status",
            "allow",
            "--ci-status",
            "success",
            "--review-status",
            "approved",
            "--manager-bin",
            "__missing_binary__",
            "--executor-bin",
            fixture_bin(),
            "--executor-arg",
            "fixture",
            "--executor-arg",
            "success",
        ],
        &out_dir,
    );

    assert!(output.status.success());
    assert_eq!(report.terminal_state.as_deref(), Some("done"));
    assert!(report.blocked_reason_codes.is_empty());

    assert!(
        report
            .stage_executions
            .iter()
            .any(|e| e.stage == "planning" && e.status == "skipped")
    );
    assert!(
        report
            .stage_executions
            .iter()
            .any(|e| e.stage == "execution" && e.status == "success")
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_planning_context_artifact_is_written() {
    let out_dir = unique_temp_dir("planning_context");
    let artifact_path = out_dir.join("planning/repo_context.json");

    let (output, report) = run_orchestrator(
        &[
            "--repo-root",
            ".",
            "--planning-context-artifact",
            artifact_path.to_str().expect("utf-8 artifact path"),
            "--policy-status",
            "allow",
            "--ci-status",
            "success",
            "--review-status",
            "approved",
        ],
        &out_dir,
    );

    assert!(output.status.success());
    assert_eq!(report.terminal_state.as_deref(), Some("done"));
    assert!(artifact_path.exists());

    let artifact_raw = fs::read_to_string(&artifact_path).expect("read planning context artifact");
    let artifact: RepoContextView =
        from_str(&artifact_raw).expect("deserialize planning context artifact");
    assert!(!artifact.repo_root.is_empty());
    assert!(
        artifact
            .detected_validation_commands
            .iter()
            .any(|c| c.command == "cargo"
                && c.args == vec!["test".to_string(), "--workspace".to_string()])
    );
    assert!(
        report
            .stage_executions
            .iter()
            .any(|e| e.stage == "planning" && e.status == "success")
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_execution_iteration_succeeds_before_budget_exhaustion() {
    let out_dir = unique_temp_dir("execution_retry_success");
    let state_file = out_dir.join("execution_retry_state.flag");
    let args = vec![
        "--policy-status".to_string(),
        "allow".to_string(),
        "--ci-status".to_string(),
        "success".to_string(),
        "--review-status".to_string(),
        "approved".to_string(),
        "--execution-max-iterations".to_string(),
        "2".to_string(),
        "--executor-bin".to_string(),
        fixture_bin().to_string(),
        "--executor-arg".to_string(),
        "fixture".to_string(),
        "--executor-arg".to_string(),
        "fail-once".to_string(),
        "--executor-arg".to_string(),
        state_file.display().to_string(),
    ];

    let (output, report) = run_orchestrator_owned(&args, &out_dir);
    assert!(output.status.success());
    assert_eq!(report.terminal_state.as_deref(), Some("done"));
    assert_eq!(
        report
            .stage_executions
            .iter()
            .filter(|e| e.stage == "execution")
            .count(),
        2
    );
    assert!(
        report
            .stage_executions
            .iter()
            .any(|e| e.stage == "execution" && e.status == "success")
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_execution_iteration_budget_exhaustion_fails_closed() {
    let out_dir = unique_temp_dir("execution_retry_budget");

    let (output, report) = run_orchestrator(
        &[
            "--policy-status",
            "allow",
            "--ci-status",
            "success",
            "--review-status",
            "approved",
            "--execution-max-iterations",
            "2",
            "--executor-bin",
            fixture_bin(),
            "--executor-arg",
            "fixture",
            "--executor-arg",
            "fail",
        ],
        &out_dir,
    );

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(report.terminal_state.as_deref(), Some("failed"));
    assert_eq!(
        report
            .stage_executions
            .iter()
            .filter(|e| e.stage == "execution" && e.status == "failed")
            .count(),
        3
    );
    assert!(
        report
            .stage_executions
            .iter()
            .any(|e| e.stage == "execution" && e.status == "failed")
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_native_validation_command_success_reaches_done() {
    let out_dir = unique_temp_dir("native_validation_success");

    let (output, report) = run_orchestrator(
        &[
            "--policy-status",
            "allow",
            "--ci-status",
            "success",
            "--review-status",
            "approved",
            "--validation-bin",
            fixture_bin(),
            "--validation-arg",
            "fixture",
            "--validation-arg",
            "success",
        ],
        &out_dir,
    );

    assert!(output.status.success());
    assert_eq!(report.terminal_state.as_deref(), Some("done"));
    assert!(
        report
            .stage_executions
            .iter()
            .any(|e| e.stage == "validation" && e.status == "success")
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_native_validation_command_failure_fails_closed() {
    let out_dir = unique_temp_dir("native_validation_failure");

    let (output, report) = run_orchestrator(
        &[
            "--policy-status",
            "allow",
            "--ci-status",
            "success",
            "--review-status",
            "approved",
            "--validation-bin",
            fixture_bin(),
            "--validation-arg",
            "fixture",
            "--validation-arg",
            "fail",
        ],
        &out_dir,
    );

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(report.terminal_state.as_deref(), Some("failed"));
    assert!(
        report
            .stage_executions
            .iter()
            .any(|e| e.stage == "validation" && e.status == "failed")
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_reviewer_remediation_cycle_recovers_to_done() {
    let out_dir = unique_temp_dir("reviewer_remediation_cycle");
    let reviewer_dir = out_dir.join("reviewer");
    fs::create_dir_all(&reviewer_dir).expect("create reviewer dir");
    let state_file = out_dir.join("reviewer_state.flag");
    let review_report = reviewer_dir.join("review_report.json");

    let args = vec![
        "--policy-status".to_string(),
        "allow".to_string(),
        "--ci-status".to_string(),
        "success".to_string(),
        "--review-status".to_string(),
        "approved".to_string(),
        "--execution-max-iterations".to_string(),
        "1".to_string(),
        "--reviewer-remediation-max-cycles".to_string(),
        "1".to_string(),
        "--executor-bin".to_string(),
        fixture_bin().to_string(),
        "--executor-arg".to_string(),
        "fixture".to_string(),
        "--executor-arg".to_string(),
        "success".to_string(),
        "--reviewer-bin".to_string(),
        fixture_bin().to_string(),
        "--reviewer-arg".to_string(),
        "fixture".to_string(),
        "--reviewer-arg".to_string(),
        "review-remediation".to_string(),
        "--reviewer-arg".to_string(),
        state_file.display().to_string(),
        "--reviewer-arg".to_string(),
        review_report.display().to_string(),
        "--reviewer-expected-artifact".to_string(),
        review_report
            .to_str()
            .expect("utf-8 review report path")
            .to_string(),
    ];

    let (output, report) = run_orchestrator_owned(&args, &out_dir);
    assert!(
        output.status.success(),
        "stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(report.terminal_state.as_deref(), Some("done"));
    assert!(
        report
            .stage_executions
            .iter()
            .filter(|e| e.stage == "execution" && e.status == "success")
            .count()
            >= 2,
        "expected execution rerun after reviewer remediation"
    );
    assert!(
        report
            .reviewer_next_steps
            .iter()
            .any(|step| step.contains("P1 [DONE] No action")),
        "reviewer_next_steps should be propagated in orchestrator report"
    );

    let _ = fs::remove_dir_all(out_dir);
}
