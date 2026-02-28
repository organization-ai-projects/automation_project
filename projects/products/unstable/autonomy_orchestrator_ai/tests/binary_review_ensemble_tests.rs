use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use common_json::from_str;

#[derive(Debug, serde::Deserialize)]
struct EnsembleReportView {
    terminal_state: Option<String>,
    blocked_reason_codes: Vec<String>,
    reviewer_verdicts: Vec<ReviewerVerdictView>,
    review_ensemble_result: Option<ReviewEnsembleResultView>,
}

#[derive(Debug, serde::Deserialize)]
struct ReviewerVerdictView {
    reviewer_id: String,
    specialty: String,
    verdict: String,
}

#[derive(Debug, serde::Deserialize)]
struct ReviewEnsembleResultView {
    passed: bool,
    reason_codes: Vec<String>,
}

fn unique_temp_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let pid = std::process::id();
    let dir =
        std::env::temp_dir().join(format!("autonomy_orchestrator_ensemble_{name}_{pid}_{nanos}"));
    fs::create_dir_all(&dir).expect("failed to create temp dir");
    dir
}

#[test]
fn ensemble_consensus_approve_path_continues_to_done() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir = unique_temp_dir("consensus");

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
        .arg("--reviewer-verdict")
        .arg("reviewer_id=r1,specialty=correctness,verdict=approve,confidence=90,weight=80")
        .arg("--reviewer-verdict")
        .arg("reviewer_id=r2,specialty=security,verdict=approve,confidence=85,weight=80")
        .arg("--reviewer-verdict")
        .arg("reviewer_id=r3,specialty=maintainability,verdict=approve,confidence=80,weight=60")
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
    let report: EnsembleReportView =
        from_str(&report_raw).expect("failed to deserialize run report");

    assert_eq!(report.terminal_state.as_deref(), Some("done"));
    assert!(report.blocked_reason_codes.is_empty());
    assert_eq!(report.reviewer_verdicts.len(), 3);
    let ensemble = report
        .review_ensemble_result
        .expect("ensemble result must be present");
    assert!(ensemble.passed);
    assert!(ensemble.reason_codes.is_empty());

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn ensemble_disagreement_security_rejection_blocks() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir = unique_temp_dir("security_reject");

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
        .arg("--reviewer-verdict")
        .arg("reviewer_id=r1,specialty=correctness,verdict=approve,confidence=100,weight=100")
        .arg("--reviewer-verdict")
        .arg("reviewer_id=r2,specialty=security,verdict=reject,confidence=90,weight=1")
        .arg("--reviewer-verdict")
        .arg("reviewer_id=r3,specialty=maintainability,verdict=approve,confidence=100,weight=100")
        .output()
        .expect("failed to execute orchestrator");

    assert_eq!(run.status.code(), Some(3));

    let report_path = out_dir.join("orchestrator_run_report.json");
    let report_raw = fs::read_to_string(&report_path).expect("failed to read run report");
    let report: EnsembleReportView =
        from_str(&report_raw).expect("failed to deserialize run report");

    assert_eq!(report.terminal_state.as_deref(), Some("blocked"));
    assert!(
        report
            .blocked_reason_codes
            .contains(&"REVIEW_ENSEMBLE_SECURITY_REJECTION".to_string())
    );
    let ensemble = report
        .review_ensemble_result
        .expect("ensemble result must be present");
    assert!(!ensemble.passed);
    assert!(ensemble
        .reason_codes
        .contains(&"REVIEW_ENSEMBLE_SECURITY_REJECTION".to_string()));

    // Verify individual verdicts are persisted.
    assert_eq!(report.reviewer_verdicts.len(), 3);
    assert!(
        report
            .reviewer_verdicts
            .iter()
            .any(|v| v.reviewer_id == "r2" && v.specialty == "security" && v.verdict == "reject")
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn ensemble_tie_disagreement_blocks_fail_closed() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir = unique_temp_dir("tie");

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
        .arg("--reviewer-verdict")
        .arg("reviewer_id=r1,specialty=correctness,verdict=approve,confidence=80,weight=50")
        .arg("--reviewer-verdict")
        .arg("reviewer_id=r2,specialty=maintainability,verdict=reject,confidence=80,weight=50")
        .output()
        .expect("failed to execute orchestrator");

    assert_eq!(run.status.code(), Some(3));

    let report_path = out_dir.join("orchestrator_run_report.json");
    let report_raw = fs::read_to_string(&report_path).expect("failed to read run report");
    let report: EnsembleReportView =
        from_str(&report_raw).expect("failed to deserialize run report");

    assert_eq!(report.terminal_state.as_deref(), Some("blocked"));
    assert!(
        report
            .blocked_reason_codes
            .contains(&"REVIEW_ENSEMBLE_TIE_FAIL_CLOSED".to_string())
    );

    let _ = fs::remove_dir_all(out_dir);
}
