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
    adaptive_policy_decisions: Vec<AdaptivePolicyDecisionView>,
    #[serde(default)]
    auto_fix_attempts: Vec<AutoFixAttemptView>,
    #[serde(default)]
    decision_rationale_codes: Vec<String>,
    stage_executions: Vec<StageExecutionView>,
    #[serde(default)]
    rollout_steps: Vec<RolloutStepView>,
    #[serde(default)]
    rollback_decision: Option<RollbackDecisionView>,
    #[serde(default)]
    planner_path_record: Option<PlannerPathRecordView>,
}

#[derive(Debug, serde::Deserialize)]
struct PlannerPathRecordView {
    selected_path: Vec<String>,
    fallback_steps_used: u32,
    reason_codes: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
struct AdaptivePolicyDecisionView {
    action: String,
    reason_code: String,
}

#[derive(Debug, serde::Deserialize)]
struct AutoFixAttemptView {
    attempt_number: u32,
    reason_code: String,
    status: String,
}

#[derive(Debug, serde::Deserialize)]
struct StageExecutionView {
    stage: String,
    status: String,
    command: String,
}

#[derive(Debug, serde::Deserialize)]
struct RolloutStepView {
    phase: String,
    reason_code: String,
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
struct RollbackDecisionView {
    triggered_at_phase: String,
    reason_code: String,
}

#[derive(Debug, serde::Deserialize)]
struct RepoContextView {
    repo_root: String,
    workspace_members: Vec<String>,
    ownership_boundaries: Vec<String>,
    hot_paths: Vec<String>,
    detected_validation_commands: Vec<RepoValidationInvocation>,
    planning_feedback: Option<PlanningFeedbackView>,
}

#[derive(Debug, serde::Deserialize)]
struct PlanningFeedbackView {
    schema_version: u32,
    terminal_state: Option<String>,
    recommended_actions: Vec<String>,
    validation_outcomes: Vec<PlanningValidationOutcomeView>,
}

#[derive(Debug, serde::Deserialize)]
struct PlanningValidationOutcomeView {
    status: String,
}

#[derive(Debug, serde::Deserialize)]
struct RepoValidationInvocation {
    command_line: RepoValidationCommandLine,
}

#[derive(Debug, serde::Deserialize)]
struct RepoValidationCommandLine {
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
    cmd.arg("--decision-contribution").arg(
        "contributor_id=matrix_default,capability=validation,vote=proceed,confidence=100,weight=100",
    );
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

fn workspace_root() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for _ in 0..4 {
        path.pop();
    }
    path
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
            workspace_root().to_str().expect("utf-8 workspace root"),
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
            .workspace_members
            .iter()
            .any(|m| m == "projects/products/unstable/autonomy_orchestrator_ai")
    );
    assert!(
        artifact
            .ownership_boundaries
            .iter()
            .any(|b| b == "projects/products/unstable")
    );
    assert!(
        artifact
            .hot_paths
            .iter()
            .any(|p| p.contains("autonomy_orchestrator_ai/src"))
    );
    assert!(artifact.detected_validation_commands.iter().any(|c| {
        c.command_line.command == "cargo"
            && c.command_line.args == vec!["test".to_string(), "--workspace".to_string()]
    }));
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
        4
    );
    assert!(
        report
            .stage_executions
            .iter()
            .any(|e| e.stage == "execution" && e.status == "failed")
    );
    assert!(
        report.adaptive_policy_decisions.iter().any(|d| {
            d.action == "increase_execution_budget"
                && d.reason_code == "ADAPTIVE_RETRY_BUDGET_INCREASED"
        }),
        "expected adaptive retry budget increase before final exhaustion"
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_execution_budget_at_cap_does_not_adapt() {
    let out_dir = unique_temp_dir("execution_retry_budget_at_cap");

    let (output, report) = run_orchestrator(
        &[
            "--policy-status",
            "allow",
            "--ci-status",
            "success",
            "--review-status",
            "approved",
            "--execution-max-iterations",
            "5",
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
    assert!(
        report
            .adaptive_policy_decisions
            .iter()
            .all(|d| d.action != "increase_execution_budget"),
        "expected no adaptive execution budget increase at hard cap"
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
fn matrix_cycle_memory_reinjects_validation_commands_on_next_run() {
    let out_dir = unique_temp_dir("cycle_memory_reinject_validation");
    let planner_output_path = out_dir.join("planning").join("planner_output.json");
    let planner_payload = format!(
        r#"{{"planner_output":{{"validation_commands":[{{"command":"{}","args":["fixture","success"]}}]}}}}"#,
        fixture_bin()
    );

    let first_args = vec![
        "--policy-status".to_string(),
        "allow".to_string(),
        "--ci-status".to_string(),
        "success".to_string(),
        "--review-status".to_string(),
        "approved".to_string(),
        "--manager-bin".to_string(),
        fixture_bin().to_string(),
        "--manager-arg".to_string(),
        "fixture".to_string(),
        "--manager-arg".to_string(),
        "write-file".to_string(),
        "--manager-arg".to_string(),
        planner_output_path.display().to_string(),
        "--manager-arg".to_string(),
        planner_payload,
        "--manager-expected-artifact".to_string(),
        planner_output_path.display().to_string(),
    ];
    let (first_output, first_report) = run_orchestrator_owned(&first_args, &out_dir);
    assert!(first_output.status.success());
    assert_eq!(first_report.terminal_state.as_deref(), Some("done"));
    assert!(
        first_report
            .stage_executions
            .iter()
            .any(|e| e.stage == "validation" && e.command == fixture_bin())
    );

    let cycle_memory_path = out_dir.join("orchestrator_cycle_memory.bin");
    assert!(
        cycle_memory_path.exists(),
        "cycle memory should be persisted after first run"
    );

    let (second_output, second_report) = run_orchestrator(
        &[
            "--policy-status",
            "allow",
            "--ci-status",
            "success",
            "--review-status",
            "approved",
        ],
        &out_dir,
    );
    assert!(second_output.status.success());
    assert_eq!(second_report.terminal_state.as_deref(), Some("done"));
    assert!(
        second_report
            .stage_executions
            .iter()
            .any(|e| e.stage == "validation" && e.command == fixture_bin()),
        "second run should reuse validation command from cycle memory"
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

#[test]
fn matrix_adaptive_remediation_budget_recovers_when_initial_budget_is_zero() {
    let out_dir = unique_temp_dir("adaptive_remediation_budget");
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
        "0".to_string(),
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
        report.adaptive_policy_decisions.iter().any(|decision| {
            decision.action == "increase_remediation_cycles"
                && decision.reason_code == "ADAPTIVE_REMEDIATION_CYCLES_INCREASED"
        }),
        "expected adaptive remediation budget decision in run report"
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_scoped_task_modifies_repo_and_passes_validation() {
    let out_dir = unique_temp_dir("scoped_repo_modify");
    let repo_dir = out_dir.join("repo");
    fs::create_dir_all(&repo_dir).expect("create repo dir");
    let generated_file = repo_dir.join("generated_scope_file.rs");

    let args = vec![
        "--repo-root".to_string(),
        repo_dir.display().to_string(),
        "--policy-status".to_string(),
        "allow".to_string(),
        "--ci-status".to_string(),
        "success".to_string(),
        "--review-status".to_string(),
        "approved".to_string(),
        "--executor-bin".to_string(),
        fixture_bin().to_string(),
        "--executor-arg".to_string(),
        "fixture".to_string(),
        "--executor-arg".to_string(),
        "write-file".to_string(),
        "--executor-arg".to_string(),
        generated_file.display().to_string(),
        "--executor-arg".to_string(),
        "pub fn scoped_fix() -> bool { true }".to_string(),
        "--validation-bin".to_string(),
        fixture_bin().to_string(),
        "--validation-arg".to_string(),
        "fixture".to_string(),
        "--validation-arg".to_string(),
        "assert-file-contains".to_string(),
        "--validation-arg".to_string(),
        generated_file.display().to_string(),
        "--validation-arg".to_string(),
        "scoped_fix".to_string(),
    ];

    let (output, report) = run_orchestrator_owned(&args, &out_dir);
    assert!(output.status.success());
    assert_eq!(report.terminal_state.as_deref(), Some("done"));
    let content = fs::read_to_string(&generated_file).expect("read generated file");
    assert!(content.contains("scoped_fix"));
    assert!(
        report
            .stage_executions
            .iter()
            .any(|e| e.stage == "execution" && e.status == "success")
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
fn matrix_delivery_dry_run_audits_steps_without_side_effects() {
    let out_dir = unique_temp_dir("delivery_dry_run");

    let (output, report) = run_orchestrator(
        &[
            "--policy-status",
            "allow",
            "--ci-status",
            "success",
            "--review-status",
            "approved",
            "--delivery-enabled",
            "--delivery-dry-run",
            "--delivery-branch",
            "feat/test-delivery-dry-run",
            "--delivery-commit-message",
            "test: dry run delivery",
            "--delivery-pr-enabled",
            "--delivery-pr-base",
            "dev",
            "--delivery-pr-title",
            "Dry run delivery PR",
            "--delivery-pr-body",
            "No side effects, only audit",
        ],
        &out_dir,
    );

    assert!(output.status.success());
    assert_eq!(report.terminal_state.as_deref(), Some("done"));
    assert!(
        report
            .stage_executions
            .iter()
            .any(|e| e.stage == "closure" && e.status == "success"),
        "expected closure delivery dry-run traces"
    );
    assert!(
        report
            .stage_executions
            .iter()
            .any(|e| e.command.contains("delivery.gh.pr.create.dry_run"))
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_delivery_pr_update_dry_run_is_audited() {
    let out_dir = unique_temp_dir("delivery_pr_update_dry_run");

    let (output, report) = run_orchestrator(
        &[
            "--policy-status",
            "allow",
            "--ci-status",
            "success",
            "--review-status",
            "approved",
            "--delivery-enabled",
            "--delivery-dry-run",
            "--delivery-pr-enabled",
            "--delivery-pr-number",
            "123",
            "--delivery-pr-title",
            "Updated title",
            "--delivery-pr-body",
            "Updated body",
        ],
        &out_dir,
    );

    assert!(output.status.success());
    assert_eq!(report.terminal_state.as_deref(), Some("done"));
    assert!(
        report
            .stage_executions
            .iter()
            .any(|e| e.command.contains("delivery.gh.pr.update.dry_run"))
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_next_actions_artifact_is_written() {
    let out_dir = unique_temp_dir("next_actions_written");
    let next_actions_path = out_dir.join("next_actions.bin");

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
        next_actions_path.exists(),
        "expected next_actions.bin to be emitted"
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_autonomous_loop_stops_on_repeated_failure_signature() {
    let out_dir = unique_temp_dir("autonomous_loop_repeated_failure");

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_autonomy_orchestrator_ai"));
    cmd.arg(&out_dir)
        .arg("--autonomous-loop")
        .arg("--autonomous-max-runs")
        .arg("5")
        .arg("--autonomous-same-error-limit")
        .arg("2")
        .arg("--policy-status")
        .arg("deny")
        .arg("--ci-status")
        .arg("success")
        .arg("--review-status")
        .arg("approved");
    let output = cmd.output().expect("execute autonomous loop run");
    assert_eq!(output.status.code(), Some(3));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Autonomous loop stopped: same failure signature repeated"),
        "expected loop stop message, got: {stdout}"
    );

    let report_path = out_dir.join("orchestrator_run_report.json");
    let report_raw = fs::read_to_string(&report_path).expect("failed to read run report");
    let report: MatrixReportView = from_str(&report_raw).expect("failed to deserialize run report");
    assert_eq!(report.terminal_state.as_deref(), Some("blocked"));

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_planning_feedback_loop_injects_previous_validation_outcomes() {
    let out_dir = unique_temp_dir("planning_feedback_loop");
    let planning_context_artifact = out_dir.join("planning/repo_context.json");
    let validation_state_file = out_dir.join("validation_fail_once.flag");

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_autonomy_orchestrator_ai"));
    cmd.arg(&out_dir)
        .arg("--autonomous-loop")
        .arg("--autonomous-max-runs")
        .arg("3")
        .arg("--autonomous-same-error-limit")
        .arg("3")
        .arg("--planning-context-artifact")
        .arg(&planning_context_artifact)
        .arg("--policy-status")
        .arg("allow")
        .arg("--ci-status")
        .arg("success")
        .arg("--review-status")
        .arg("approved")
        .arg("--executor-bin")
        .arg(fixture_bin())
        .arg("--executor-arg")
        .arg("fixture")
        .arg("--executor-arg")
        .arg("success")
        .arg("--validation-bin")
        .arg(fixture_bin())
        .arg("--validation-arg")
        .arg("fixture")
        .arg("--validation-arg")
        .arg("fail-once")
        .arg("--validation-arg")
        .arg(&validation_state_file);
    let output = cmd.output().expect("execute feedback loop run");
    assert_eq!(
        output.status.code(),
        Some(0),
        "stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let artifact_raw = fs::read_to_string(&planning_context_artifact)
        .expect("read planning context with feedback artifact");
    let artifact: RepoContextView =
        from_str(&artifact_raw).expect("deserialize planning context with feedback artifact");
    let feedback = artifact
        .planning_feedback
        .expect("expected planning_feedback to be injected");
    assert_eq!(feedback.schema_version, 1);
    assert_eq!(feedback.terminal_state.as_deref(), Some("failed"));
    assert!(
        feedback
            .recommended_actions
            .iter()
            .any(|action| action.contains("Inspect failed stage execution")),
        "expected failure remediation recommendation in planning feedback"
    );
    assert!(
        feedback
            .validation_outcomes
            .iter()
            .any(|outcome| outcome.status == "failed"),
        "expected failed validation outcome in planning feedback"
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_autonomous_loop_no_progress_stops_with_noop_signature() {
    let out_dir = unique_temp_dir("autonomous_loop_no_progress");

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_autonomy_orchestrator_ai"));
    cmd.arg(&out_dir)
        .arg("--autonomous-loop")
        .arg("--autonomous-max-runs")
        .arg("5")
        .arg("--autonomous-same-error-limit")
        .arg("10")
        .arg("--policy-status")
        .arg("deny")
        .arg("--ci-status")
        .arg("success")
        .arg("--review-status")
        .arg("approved");
    let output = cmd.output().expect("execute autonomous loop run");
    assert_eq!(output.status.code(), Some(3));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Autonomous loop stopped: no-op signature repeated"),
        "expected no-op stop message, got: {stdout}"
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_autonomous_loop_progress_then_done() {
    let out_dir = unique_temp_dir("autonomous_loop_progress_then_done");
    let state_file = out_dir.join("autonomous_fail_once_state.flag");

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_autonomy_orchestrator_ai"));
    cmd.arg(&out_dir)
        .arg("--autonomous-loop")
        .arg("--autonomous-max-runs")
        .arg("3")
        .arg("--autonomous-same-error-limit")
        .arg("3")
        .arg("--policy-status")
        .arg("allow")
        .arg("--ci-status")
        .arg("success")
        .arg("--review-status")
        .arg("approved")
        .arg("--decision-contribution")
        .arg(
            "contributor_id=loop_default,capability=execution,vote=proceed,confidence=100,weight=100",
        )
        .arg("--execution-max-iterations")
        .arg("1")
        .arg("--executor-bin")
        .arg(fixture_bin())
        .arg("--executor-arg")
        .arg("fixture")
        .arg("--executor-arg")
        .arg("fail-once")
        .arg("--executor-arg")
        .arg(state_file);
    let output = cmd.output().expect("execute autonomous loop run");
    assert_eq!(
        output.status.code(),
        Some(0),
        "stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let report_path = out_dir.join("orchestrator_run_report.json");
    let report_raw = fs::read_to_string(&report_path).expect("failed to read run report");
    let report: MatrixReportView = from_str(&report_raw).expect("failed to deserialize run report");
    assert_eq!(report.terminal_state.as_deref(), Some("done"));

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_decision_conflict_tie_fail_closed() {
    let out_dir = unique_temp_dir("decision_tie_fail_closed");
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");

    let output = Command::new(bin)
        .arg(&out_dir)
        .arg("--policy-status")
        .arg("allow")
        .arg("--ci-status")
        .arg("success")
        .arg("--review-status")
        .arg("approved")
        .arg("--decision-contribution")
        .arg("contributor_id=a,capability=planning,vote=proceed,confidence=50,weight=50")
        .arg("--decision-contribution")
        .arg("contributor_id=b,capability=execution,vote=block,confidence=50,weight=50")
        .arg("--decision-contribution")
        .arg("contributor_id=c,capability=validation,vote=escalate,confidence=50,weight=50")
        .output()
        .expect("execute tie fail-closed run");

    assert_eq!(output.status.code(), Some(3));
    let report_path = out_dir.join("orchestrator_run_report.json");
    let report_raw = fs::read_to_string(&report_path).expect("failed to read run report");
    let report: MatrixReportView = from_str(&report_raw).expect("failed to deserialize run report");
    assert_eq!(report.terminal_state.as_deref(), Some("blocked"));
    assert!(
        report
            .blocked_reason_codes
            .contains(&"DECISION_TIE_FAIL_CLOSED".to_string())
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_decision_low_confidence_blocks() {
    let out_dir = unique_temp_dir("decision_low_confidence");
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");

    let output = Command::new(bin)
        .arg(&out_dir)
        .arg("--policy-status")
        .arg("allow")
        .arg("--ci-status")
        .arg("success")
        .arg("--review-status")
        .arg("approved")
        .arg("--decision-threshold")
        .arg("70")
        .arg("--decision-contribution")
        .arg("contributor_id=a,capability=planning,vote=proceed,confidence=55,weight=1")
        .arg("--decision-contribution")
        .arg("contributor_id=b,capability=review,vote=block,confidence=45,weight=1")
        .output()
        .expect("execute low confidence run");

    assert_eq!(output.status.code(), Some(3));
    let report_path = out_dir.join("orchestrator_run_report.json");
    let report_raw = fs::read_to_string(&report_path).expect("failed to read run report");
    let report: MatrixReportView = from_str(&report_raw).expect("failed to deserialize run report");
    assert_eq!(report.terminal_state.as_deref(), Some("blocked"));
    assert!(
        report
            .blocked_reason_codes
            .contains(&"DECISION_CONFIDENCE_BELOW_THRESHOLD".to_string())
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_decision_no_contributions_blocks_fail_closed() {
    let out_dir = unique_temp_dir("decision_no_contributions");
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");

    let output = Command::new(bin)
        .arg(&out_dir)
        .arg("--policy-status")
        .arg("allow")
        .arg("--ci-status")
        .arg("success")
        .arg("--review-status")
        .arg("approved")
        .arg("--decision-require-contributions")
        .output()
        .expect("execute no contribution run");

    assert_eq!(output.status.code(), Some(3));
    let report_path = out_dir.join("orchestrator_run_report.json");
    let report_raw = fs::read_to_string(&report_path).expect("failed to read run report");
    let report: MatrixReportView = from_str(&report_raw).expect("failed to deserialize run report");
    assert_eq!(report.terminal_state.as_deref(), Some("blocked"));
    assert!(
        report
            .blocked_reason_codes
            .contains(&"DECISION_NO_CONTRIBUTIONS".to_string())
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_rollout_enabled_healthy_progression_to_full() {
    let out_dir = unique_temp_dir("rollout_healthy");

    let (output, report) = run_orchestrator(
        &[
            "--policy-status",
            "allow",
            "--ci-status",
            "success",
            "--review-status",
            "approved",
            "--rollout-enabled",
        ],
        &out_dir,
    );

    assert!(output.status.success());
    assert_eq!(report.terminal_state.as_deref(), Some("done"));
    assert!(report.rollback_decision.is_none());
    assert_eq!(report.rollout_steps.len(), 3);
    assert_eq!(report.rollout_steps[0].phase, "canary");
    assert_eq!(report.rollout_steps[1].phase, "partial");
    assert_eq!(report.rollout_steps[2].phase, "full");
    assert!(
        report
            .rollout_steps
            .iter()
            .all(|s| s.reason_code == "ROLLOUT_PHASE_ADVANCED")
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_planner_v2_successful_fallback_flow() {
    let out_dir = unique_temp_dir("planner_v2_fallback_ok");
    let planner_output_path = out_dir.join("planner_output.json");
    // Graph: start →(ON_FAIL) middle →(primary) end; 1 fallback within budget of 3
    fs::write(
        &planner_output_path,
        r#"{
            "planner_output": {
                "planner_nodes": [
                    {"id": "start", "action": "step_start"},
                    {"id": "middle", "action": "step_middle"},
                    {"id": "end", "action": "step_end"}
                ],
                "planner_edges": [
                    {"from": "start", "to": "middle", "condition_code": "ON_FAIL"},
                    {"from": "middle", "to": "end", "condition_code": ""}
                ]
            }
        }"#,
    )
    .expect("write planner output artifact");

    let (output, report) = run_orchestrator(
        &[
            "--policy-status",
            "allow",
            "--ci-status",
            "success",
            "--review-status",
            "approved",
            "--manager-bin",
            "true",
            "--manager-expected-artifact",
            planner_output_path.to_str().unwrap(),
            "--planner-fallback-max-steps",
            "3",
        ],
        &out_dir,
    );

    assert_eq!(output.status.code(), Some(0), "expected exit 0");
    assert_eq!(report.terminal_state.as_deref(), Some("done"));
    let record = report
        .planner_path_record
        .expect("planner_path_record must be set");
    assert_eq!(record.selected_path, vec!["start", "middle", "end"]);
    assert_eq!(record.fallback_steps_used, 1);
    assert!(
        record
            .reason_codes
            .contains(&"PLANNER_FALLBACK_STEP_APPLIED".to_string())
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_rollout_disabled_produces_no_rollout_telemetry() {
    let out_dir = unique_temp_dir("rollout_disabled");

    let (output, report) = run_orchestrator(
        &[
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
    assert!(report.rollout_steps.is_empty());
    assert!(report.rollback_decision.is_none());
    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_planner_v2_exhausted_fallback_budget_fails_closed() {
    let out_dir = unique_temp_dir("planner_v2_budget_exhausted");
    let planner_output_path = out_dir.join("planner_output.json");
    // Graph: a →(ON_FAIL) b →(ON_FAIL) c; budget = 1 → exhausted before reaching c
    fs::write(
        &planner_output_path,
        r#"{
            "planner_output": {
                "planner_nodes": [
                    {"id": "a", "action": "step_a"},
                    {"id": "b", "action": "step_b"},
                    {"id": "c", "action": "step_c"}
                ],
                "planner_edges": [
                    {"from": "a", "to": "b", "condition_code": "ON_FAIL"},
                    {"from": "b", "to": "c", "condition_code": "ON_FAIL"}
                ]
            }
        }"#,
    )
    .expect("write planner output artifact");

    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let output = Command::new(bin)
        .arg(&out_dir)
        .arg("--decision-contribution")
        .arg("contributor_id=default,capability=governance,vote=proceed,confidence=100,weight=100")
        .arg("--manager-bin")
        .arg("true")
        .arg("--manager-expected-artifact")
        .arg(planner_output_path.to_str().unwrap())
        .arg("--planner-fallback-max-steps")
        .arg("1")
        .output()
        .expect("execute planner v2 budget exhausted run");

    assert_eq!(output.status.code(), Some(1), "expected exit 1 (failed)");

    let report_path = out_dir.join("orchestrator_run_report.json");
    let report_raw = fs::read_to_string(&report_path).expect("failed to read run report");
    let report: MatrixReportView = from_str(&report_raw).expect("failed to deserialize run report");

    assert_eq!(report.terminal_state.as_deref(), Some("failed"));
    let record = report
        .planner_path_record
        .expect("planner_path_record must be set");
    assert_eq!(record.fallback_steps_used, 1);
    assert!(
        record
            .reason_codes
            .contains(&"PLANNER_FALLBACK_BUDGET_EXHAUSTED".to_string())
    );
    assert!(
        report
            .stage_executions
            .iter()
            .any(|e| e.command == "planning.planner_v2.select_path" && e.status == "failed")
    );

    let _ = fs::remove_dir_all(out_dir);
}
