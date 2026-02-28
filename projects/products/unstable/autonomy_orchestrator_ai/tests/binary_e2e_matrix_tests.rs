mod common;

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

use common::json_fields::{json_field_array, json_field_str, json_field_u64};
use common::path_factory::{unique_temp_dir, workspace_root_from_manifest};
use common::process_runner::run_autonomy_orchestrator;
use common_json::{Json, JsonAccess, from_str};

#[derive(Debug, serde::Deserialize)]
struct MatrixReportView {
    terminal_state: Option<String>,
    blocked_reason_codes: Vec<String>,
    reviewer_next_steps: Vec<String>,
    adaptive_policy_decisions: Vec<Json>,
    stage_executions: Vec<Json>,
    #[serde(default)]
    rollout_steps: Vec<Json>,
    #[serde(default)]
    rollback_decision: Option<Json>,
    #[serde(default)]
    planner_path_record: Option<Json>,
}

#[derive(Debug, serde::Serialize)]
struct CheckpointFixture {
    run_id: String,
    completed_stages: Vec<String>,
    terminal_state: Option<String>,
    updated_at_unix_secs: u64,
}

fn run_orchestrator(args: &[&str], out_dir: &PathBuf) -> (Output, MatrixReportView) {
    let mut full_args = vec![
        "--decision-contribution",
        "contributor_id=matrix_default,capability=validation,vote=proceed,confidence=100,weight=100",
    ];
    full_args.extend_from_slice(args);
    let output = run_autonomy_orchestrator(out_dir, &full_args);

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
    workspace_root_from_manifest()
}

fn run_with_default_gates(extra_args: &[&str], out_dir: &PathBuf) -> (Output, MatrixReportView) {
    let mut args = vec![
        "--policy-status",
        "allow",
        "--ci-status",
        "success",
        "--review-status",
        "approved",
    ];
    args.extend_from_slice(extra_args);
    run_orchestrator(&args, out_dir)
}

fn has_stage_status(report: &MatrixReportView, stage: &str, status: &str) -> bool {
    report.stage_executions.iter().any(|execution| {
        json_field_str(execution, "stage") == Some(stage)
            && json_field_str(execution, "status") == Some(status)
    })
}

fn has_command_status(report: &MatrixReportView, command: &str, status: &str) -> bool {
    report.stage_executions.iter().any(|execution| {
        json_field_str(execution, "command") == Some(command)
            && json_field_str(execution, "status") == Some(status)
    })
}

#[test]
fn matrix_happy_path_reaches_done() {
    let out_dir = unique_temp_dir("happy_path");

    let (output, report) = run_with_default_gates(
        &[
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
            .all(|e| json_field_str(e, "status") == Some("success"))
    );
    assert!(has_stage_status(&report, "validation", "success"));

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

    let (output, report) = run_with_default_gates(
        &[
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
    assert_eq!(
        json_field_str(&report.stage_executions[0], "status"),
        Some("timeout")
    );

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

    let (output, report) = run_with_default_gates(
        &[
            "--resume",
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

    assert!(has_stage_status(&report, "planning", "skipped"));
    assert!(has_stage_status(&report, "execution", "success"));

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_planning_context_artifact_is_written() {
    let out_dir = unique_temp_dir("planning_context");
    let artifact_path = out_dir.join("planning/repo_context.json");

    let (output, report) = run_with_default_gates(
        &[
            "--repo-root",
            workspace_root().to_str().expect("utf-8 workspace root"),
            "--planning-context-artifact",
            artifact_path.to_str().expect("utf-8 artifact path"),
        ],
        &out_dir,
    );

    assert!(output.status.success());
    assert_eq!(report.terminal_state.as_deref(), Some("done"));
    assert!(artifact_path.exists());

    let artifact_raw = fs::read_to_string(&artifact_path).expect("read planning context artifact");
    let artifact: Json = from_str(&artifact_raw).expect("deserialize planning context artifact");
    let repo_root = artifact
        .get_field("repo_root")
        .and_then(|v| v.as_str_strict())
        .expect("repo_root must be a string");
    assert!(!repo_root.is_empty());

    let workspace_members = artifact
        .get_field("workspace_members")
        .and_then(|v| v.as_array_strict())
        .expect("workspace_members must be an array");
    assert!(workspace_members.iter().any(|m| {
        m.as_str_strict()
            .is_ok_and(|s| s == "projects/products/unstable/autonomy_orchestrator_ai")
    }));

    let ownership_boundaries = artifact
        .get_field("ownership_boundaries")
        .and_then(|v| v.as_array_strict())
        .expect("ownership_boundaries must be an array");
    assert!(ownership_boundaries.iter().any(|b| {
        b.as_str_strict()
            .is_ok_and(|s| s == "projects/products/unstable")
    }));

    let hot_paths = artifact
        .get_field("hot_paths")
        .and_then(|v| v.as_array_strict())
        .expect("hot_paths must be an array");
    assert!(hot_paths.iter().any(|p| {
        p.as_str_strict()
            .is_ok_and(|s| s.contains("autonomy_orchestrator_ai/src"))
    }));

    let detected_commands = artifact
        .get_field("detected_validation_commands")
        .and_then(|v| v.as_array_strict())
        .expect("detected_validation_commands must be an array");
    assert!(detected_commands.iter().any(|entry| {
        let command = entry
            .get_field("command_line")
            .and_then(|c| c.get_field("command"))
            .and_then(|v| v.as_str_strict());
        let args = entry
            .get_field("command_line")
            .and_then(|c| c.get_field("args"))
            .and_then(|v| v.as_array_strict());
        if command.ok() != Some("cargo") {
            return false;
        }
        match args {
            Ok(values) => {
                values.len() == 2
                    && values[0].as_str_strict().ok() == Some("test")
                    && values[1].as_str_strict().ok() == Some("--workspace")
            }
            Err(_) => false,
        }
    }));
    assert!(has_stage_status(&report, "planning", "success"));

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
            .filter(|e| json_field_str(e, "stage") == Some("execution"))
            .count(),
        2
    );
    assert!(report.stage_executions.iter().any(|e| {
        json_field_str(e, "stage") == Some("execution")
            && json_field_str(e, "status") == Some("success")
    }));

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
            .filter(|e| {
                json_field_str(e, "stage") == Some("execution")
                    && json_field_str(e, "status") == Some("failed")
            })
            .count(),
        4
    );
    assert!(report.stage_executions.iter().any(|e| {
        json_field_str(e, "stage") == Some("execution")
            && json_field_str(e, "status") == Some("failed")
    }));
    assert!(
        report.adaptive_policy_decisions.iter().any(|d| {
            json_field_str(d, "action") == Some("increase_execution_budget")
                && json_field_str(d, "reason_code") == Some("ADAPTIVE_RETRY_BUDGET_INCREASED")
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
            .all(|d| json_field_str(d, "action") != Some("increase_execution_budget")),
        "expected no adaptive execution budget increase at hard cap"
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_native_validation_command_success_reaches_done() {
    let out_dir = unique_temp_dir("native_validation_success");

    let (output, report) = run_with_default_gates(
        &[
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
    assert!(has_stage_status(&report, "validation", "success"));

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn matrix_native_validation_command_failure_fails_closed() {
    let out_dir = unique_temp_dir("native_validation_failure");

    let (output, report) = run_with_default_gates(
        &[
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
    assert!(has_stage_status(&report, "validation", "failed"));

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
    assert!(has_command_status(&first_report, fixture_bin(), "success"));

    let cycle_memory_path = out_dir.join("orchestrator_cycle_memory.bin");
    assert!(
        cycle_memory_path.exists(),
        "cycle memory should be persisted after first run"
    );

    let (second_output, second_report) = run_with_default_gates(&[], &out_dir);
    assert!(second_output.status.success());
    assert_eq!(second_report.terminal_state.as_deref(), Some("done"));
    assert!(
        second_report.stage_executions.iter().any(|e| {
            json_field_str(e, "stage") == Some("validation")
                && json_field_str(e, "command") == Some(fixture_bin())
        }),
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
            .filter(|e| {
                json_field_str(e, "stage") == Some("execution")
                    && json_field_str(e, "status") == Some("success")
            })
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
            json_field_str(decision, "action") == Some("increase_remediation_cycles")
                && json_field_str(decision, "reason_code")
                    == Some("ADAPTIVE_REMEDIATION_CYCLES_INCREASED")
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
    assert!(report.stage_executions.iter().any(|e| {
        json_field_str(e, "stage") == Some("execution")
            && json_field_str(e, "status") == Some("success")
    }));
    assert!(report.stage_executions.iter().any(|e| {
        json_field_str(e, "stage") == Some("validation")
            && json_field_str(e, "status") == Some("success")
    }));

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
        report.stage_executions.iter().any(|e| {
            json_field_str(e, "stage") == Some("closure")
                && json_field_str(e, "status") == Some("success")
        }),
        "expected closure delivery dry-run traces"
    );
    assert!(report.stage_executions.iter().any(|e| {
        matches!(
            json_field_str(e, "command"),
            Some(command) if command.contains("delivery.gh.pr.create.dry_run")
        )
    }));

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
    assert!(report.stage_executions.iter().any(|e| {
        matches!(
            json_field_str(e, "command"),
            Some(command) if command.contains("delivery.gh.pr.update.dry_run")
        )
    }));

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
    let artifact: Json =
        from_str(&artifact_raw).expect("deserialize planning context with feedback artifact");
    let feedback = artifact
        .get_field("planning_feedback")
        .expect("expected planning_feedback to be injected");
    assert_eq!(
        feedback
            .get_field("schema_version")
            .and_then(|v| v.as_u64_strict())
            .expect("schema_version must be u64"),
        1
    );
    assert_eq!(
        feedback
            .get_field("terminal_state")
            .and_then(|v| v.as_str_strict())
            .expect("terminal_state must be string"),
        "failed"
    );
    let recommended_actions = feedback
        .get_field("recommended_actions")
        .and_then(|v| v.as_array_strict())
        .expect("recommended_actions must be array");
    assert!(
        recommended_actions.iter().any(|action| {
            action
                .as_str_strict()
                .is_ok_and(|s| s.contains("Inspect failed stage execution"))
        }),
        "expected failure remediation recommendation in planning feedback"
    );
    let validation_outcomes = feedback
        .get_field("validation_outcomes")
        .and_then(|v| v.as_array_strict())
        .expect("validation_outcomes must be array");
    assert!(
        validation_outcomes.iter().any(|outcome| {
            outcome
                .get_field("status")
                .and_then(|v| v.as_str_strict())
                .is_ok_and(|s| s == "failed")
        }),
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
    assert_eq!(
        json_field_str(&report.rollout_steps[0], "phase"),
        Some("canary")
    );
    assert_eq!(
        json_field_str(&report.rollout_steps[1], "phase"),
        Some("partial")
    );
    assert_eq!(
        json_field_str(&report.rollout_steps[2], "phase"),
        Some("full")
    );
    assert!(
        report
            .rollout_steps
            .iter()
            .all(|s| json_field_str(s, "reason_code") == Some("ROLLOUT_PHASE_ADVANCED"))
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
    let selected_path = json_field_array(&record, "selected_path").expect("selected_path array");
    assert_eq!(
        selected_path
            .iter()
            .map(|v: &Json| v.as_str_strict().expect("selected_path item string"))
            .collect::<Vec<_>>(),
        vec!["start", "middle", "end"]
    );
    assert_eq!(json_field_u64(&record, "fallback_steps_used"), Some(1));
    let reason_codes = json_field_array(&record, "reason_codes").expect("reason_codes array");
    assert!(
        reason_codes
            .iter()
            .any(|v: &Json| v.as_str_strict().ok() == Some("PLANNER_FALLBACK_STEP_APPLIED"))
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
    assert_eq!(json_field_u64(&record, "fallback_steps_used"), Some(1));
    let reason_codes = json_field_array(&record, "reason_codes").expect("reason_codes array");
    assert!(
        reason_codes
            .iter()
            .any(|v: &Json| v.as_str_strict().ok() == Some("PLANNER_FALLBACK_BUDGET_EXHAUSTED"))
    );
    assert!(report.stage_executions.iter().any(|e| {
        json_field_str(e, "command") == Some("planning.planner_v2.select_path")
            && json_field_str(e, "status") == Some("failed")
    }));

    let _ = fs::remove_dir_all(out_dir);
}
