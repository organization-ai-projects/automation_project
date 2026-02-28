use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use common_json::from_str;

#[derive(Debug, serde::Deserialize)]
struct GateReportView {
    terminal_state: Option<String>,
    blocked_reason_codes: Vec<String>,
    gate_decisions: Vec<GateDecisionView>,
    hard_gate_results: Vec<HardGateResultView>,
}

#[derive(Debug, serde::Deserialize)]
struct GateDecisionView {
    gate: String,
    passed: bool,
    reason_code: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct HardGateResultView {
    id: String,
    passed: bool,
    reason_code: String,
}

fn unique_temp_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let pid = std::process::id();
    let dir = std::env::temp_dir().join(format!("autonomy_orchestrator_{name}_{pid}_{nanos}"));
    fs::create_dir_all(&dir).expect("failed to create temp dir");
    dir
}

fn bin_path() -> &'static str {
    env!("CARGO_BIN_EXE_autonomy_orchestrator_ai")
}

#[test]
fn default_gate_signals_fail_closed_with_reason_codes() {
    let bin = bin_path();
    let out_dir = unique_temp_dir("gates_default");

    let run = Command::new(bin)
        .arg(&out_dir)
        .output()
        .expect("failed to execute orchestrator");

    assert_eq!(run.status.code(), Some(3));

    let report_path = out_dir.join("orchestrator_run_report.json");
    let report_raw = fs::read_to_string(&report_path).expect("failed to read run report");
    let report: GateReportView = from_str(&report_raw).expect("failed to deserialize run report");

    assert_eq!(report.terminal_state.as_deref(), Some("blocked"));
    assert!(
        report
            .blocked_reason_codes
            .contains(&"GATE_POLICY_DENIED_OR_UNKNOWN".to_string())
    );
    assert!(
        report
            .blocked_reason_codes
            .contains(&"GATE_CI_NOT_SUCCESS".to_string())
    );
    assert!(
        report
            .blocked_reason_codes
            .contains(&"GATE_REVIEW_NOT_APPROVED".to_string())
    );

    assert_eq!(report.gate_decisions.len(), 3);
    assert!(
        report
            .gate_decisions
            .iter()
            .any(|d| d.gate == "policy" && !d.passed)
    );
    assert!(
        report
            .gate_decisions
            .iter()
            .any(|d| d.gate == "ci" && !d.passed)
    );
    assert!(
        report
            .gate_decisions
            .iter()
            .any(|d| d.gate == "review" && !d.passed)
    );
    assert!(report.gate_decisions.iter().all(|d| {
        d.reason_code
            .as_ref()
            .map(|c| !c.is_empty())
            .unwrap_or(false)
    }));

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn explicit_pass_gate_signals_reach_done() {
    let bin = bin_path();
    let out_dir = unique_temp_dir("gates_pass");

    let run = Command::new(bin)
        .arg(&out_dir)
        .arg("--policy-status")
        .arg("allow")
        .arg("--ci-status")
        .arg("success")
        .arg("--review-status")
        .arg("approved")
        .arg("--decision-contribution")
        .arg(
            "contributor_id=gate_test,capability=validation,vote=proceed,confidence=100,weight=100",
        )
        .output()
        .expect("failed to execute orchestrator");

    assert!(
        run.status.success(),
        "run failed: stdout={} stderr={}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );

    let report_path = out_dir.join("orchestrator_run_report.json");
    let report_raw = fs::read_to_string(&report_path).expect("failed to read run report");
    let report: GateReportView = from_str(&report_raw).expect("failed to deserialize run report");

    assert_eq!(report.terminal_state.as_deref(), Some("done"));
    assert!(report.blocked_reason_codes.is_empty());
    assert!(report.gate_decisions.iter().all(|d| d.passed));
    assert!(report.hard_gate_results.iter().all(|r| r.passed));

    let _ = fs::remove_dir_all(out_dir);
}

// --- Hard gate E2E tests ---

fn run_with_executor_arg(out_dir: &PathBuf, executor_arg: &str) -> std::process::Output {
    let bin = bin_path();
    Command::new(bin)
        .arg(out_dir)
        .arg("--policy-status")
        .arg("allow")
        .arg("--ci-status")
        .arg("success")
        .arg("--review-status")
        .arg("approved")
        .arg("--executor-bin")
        .arg("true")
        .arg("--executor-arg")
        .arg(executor_arg)
        .output()
        .expect("failed to execute orchestrator")
}

fn load_report(out_dir: &PathBuf) -> GateReportView {
    let report_path = out_dir.join("orchestrator_run_report.json");
    let report_raw = fs::read_to_string(&report_path).expect("failed to read run report");
    from_str(&report_raw).expect("failed to deserialize run report")
}

#[test]
fn hard_gate_secrets_category_blocks_run_preexecution() {
    let out_dir = unique_temp_dir("hard_gate_secrets");
    let run = run_with_executor_arg(&out_dir, "dump-secrets");

    assert_eq!(
        run.status.code(),
        Some(3),
        "expected blocked exit code 3, stdout={} stderr={}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );

    let report = load_report(&out_dir);
    assert_eq!(report.terminal_state.as_deref(), Some("blocked"));
    assert!(
        report
            .blocked_reason_codes
            .contains(&"HARD_GATE_SECRET_POLICY_VIOLATION".to_string()),
        "expected HARD_GATE_SECRET_POLICY_VIOLATION in {:?}",
        report.blocked_reason_codes
    );
    assert!(
        report
            .hard_gate_results
            .iter()
            .any(|r| !r.passed && r.reason_code == "HARD_GATE_SECRET_POLICY_VIOLATION"),
        "expected failing hard gate result for secrets"
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn hard_gate_auth_category_blocks_run_preexecution() {
    let out_dir = unique_temp_dir("hard_gate_auth");
    let run = run_with_executor_arg(&out_dir, "modify-sudoers");

    assert_eq!(run.status.code(), Some(3));

    let report = load_report(&out_dir);
    assert_eq!(report.terminal_state.as_deref(), Some("blocked"));
    assert!(
        report
            .blocked_reason_codes
            .contains(&"HARD_GATE_AUTH_POLICY_VIOLATION".to_string()),
        "expected HARD_GATE_AUTH_POLICY_VIOLATION in {:?}",
        report.blocked_reason_codes
    );
    assert!(
        report
            .hard_gate_results
            .iter()
            .any(|r| !r.passed && r.reason_code == "HARD_GATE_AUTH_POLICY_VIOLATION")
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn hard_gate_git_history_category_blocks_run_preexecution() {
    let out_dir = unique_temp_dir("hard_gate_git_history");
    let run = run_with_executor_arg(&out_dir, "filter-branch");

    assert_eq!(run.status.code(), Some(3));

    let report = load_report(&out_dir);
    assert_eq!(report.terminal_state.as_deref(), Some("blocked"));
    assert!(
        report
            .blocked_reason_codes
            .contains(&"HARD_GATE_GIT_HISTORY_REWRITE_FORBIDDEN".to_string()),
        "expected HARD_GATE_GIT_HISTORY_REWRITE_FORBIDDEN in {:?}",
        report.blocked_reason_codes
    );
    assert!(
        report
            .hard_gate_results
            .iter()
            .any(|r| !r.passed && r.reason_code == "HARD_GATE_GIT_HISTORY_REWRITE_FORBIDDEN")
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn hard_gate_infra_destructive_category_blocks_run_preexecution() {
    let out_dir = unique_temp_dir("hard_gate_infra");
    let run = run_with_executor_arg(&out_dir, "terraform-destroy");

    assert_eq!(run.status.code(), Some(3));

    let report = load_report(&out_dir);
    assert_eq!(report.terminal_state.as_deref(), Some("blocked"));
    assert!(
        report
            .blocked_reason_codes
            .contains(&"HARD_GATE_INFRA_DESTRUCTIVE_OPERATION".to_string()),
        "expected HARD_GATE_INFRA_DESTRUCTIVE_OPERATION in {:?}",
        report.blocked_reason_codes
    );
    assert!(
        report
            .hard_gate_results
            .iter()
            .any(|r| !r.passed && r.reason_code == "HARD_GATE_INFRA_DESTRUCTIVE_OPERATION")
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn hard_gate_results_persisted_in_run_report_artifact() {
    let out_dir = unique_temp_dir("hard_gate_persist");
    let run = run_with_executor_arg(&out_dir, "dump-secrets");

    assert_eq!(run.status.code(), Some(3));

    let report = load_report(&out_dir);
    assert!(
        !report.hard_gate_results.is_empty(),
        "expected hard_gate_results to be populated in run report"
    );
    assert!(
        report
            .hard_gate_results
            .iter()
            .any(|r| !r.passed && r.reason_code == "HARD_GATE_SECRET_POLICY_VIOLATION"),
        "expected failing secrets hard gate in results"
    );
    assert!(
        report
            .hard_gate_results
            .iter()
            .all(|r| !r.id.is_empty() && !r.reason_code.is_empty()),
        "all hard gate results must have non-empty id and reason_code"
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn external_hard_gate_file_appends_and_blocks() {
    let out_dir = unique_temp_dir("hard_gate_external");
    let rules_file = out_dir.join("custom_gates.json");

    fs::write(
        &rules_file,
        r#"[{"id":"ext-custom-1","category":"infra_destructive","pattern":"custom-destroy","mode":"match_any_invocation_arg"}]"#,
    )
    .expect("write rules file");

    let bin = bin_path();
    let run = Command::new(bin)
        .arg(&out_dir)
        .arg(&rules_file)
        .arg("--policy-status")
        .arg("allow")
        .arg("--ci-status")
        .arg("success")
        .arg("--review-status")
        .arg("approved")
        .arg("--executor-bin")
        .arg("true")
        .arg("--executor-arg")
        .arg("custom-destroy")
        .output()
        .expect("failed to execute orchestrator");

    assert_eq!(
        run.status.code(),
        Some(3),
        "expected blocked exit code 3, stdout={} stderr={}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );

    let report = load_report(&out_dir);
    assert_eq!(report.terminal_state.as_deref(), Some("blocked"));
    assert!(
        report
            .blocked_reason_codes
            .contains(&"HARD_GATE_INFRA_DESTRUCTIVE_OPERATION".to_string()),
        "expected HARD_GATE_INFRA_DESTRUCTIVE_OPERATION in {:?}",
        report.blocked_reason_codes
    );
    assert!(
        report
            .hard_gate_results
            .iter()
            .any(|r| r.id == "ext-custom-1" && !r.passed),
        "expected external rule to appear and fail in hard_gate_results"
    );

    let _ = fs::remove_dir_all(out_dir);
}
