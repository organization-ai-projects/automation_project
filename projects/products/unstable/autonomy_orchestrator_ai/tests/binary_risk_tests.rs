use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use common_json::from_str;

#[derive(Debug, serde::Deserialize)]
struct RiskReportView {
    terminal_state: Option<String>,
    blocked_reason_codes: Vec<String>,
    decision_rationale_codes: Vec<String>,
    pr_risk_breakdown: Option<PrRiskBreakdownView>,
}

#[derive(Debug, serde::Deserialize)]
struct PrRiskBreakdownView {
    total_score: u16,
    factors: Vec<PrRiskFactorView>,
    eligible_for_auto_merge: bool,
}

#[derive(Debug, serde::Deserialize)]
struct PrRiskFactorView {
    name: String,
    score: u16,
    #[allow(dead_code)]
    rationale: String,
}

fn unique_temp_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let pid = std::process::id();
    let dir = std::env::temp_dir().join(format!("autonomy_orch_risk_{name}_{pid}_{nanos}"));
    fs::create_dir_all(&dir).expect("failed to create temp dir");
    dir
}

// --- Integration: risk breakdown is always persisted in run report ---

#[test]
fn risk_breakdown_persisted_in_run_report_on_done() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir = unique_temp_dir("risk_done");

    let output = Command::new(bin)
        .arg(&out_dir)
        .arg("--policy-status")
        .arg("allow")
        .arg("--ci-status")
        .arg("success")
        .arg("--review-status")
        .arg("approved")
        .arg("--decision-contribution")
        .arg(
            "contributor_id=risk_test,capability=validation,vote=proceed,confidence=100,weight=100",
        )
        .output()
        .expect("failed to execute orchestrator");

    assert!(
        output.status.success(),
        "expected exit 0, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let report_raw =
        fs::read_to_string(out_dir.join("orchestrator_run_report.json")).expect("report missing");
    let report: RiskReportView = from_str(&report_raw).expect("failed to parse report");

    assert_eq!(report.terminal_state.as_deref(), Some("done"));

    let breakdown = report
        .pr_risk_breakdown
        .expect("pr_risk_breakdown must be present");
    assert_eq!(breakdown.factors.len(), 5);
    assert_eq!(breakdown.total_score, 0);
    assert!(breakdown.eligible_for_auto_merge);

    let factor_names: Vec<&str> = breakdown.factors.iter().map(|f| f.name.as_str()).collect();
    assert!(factor_names.contains(&"gate_quality"));
    assert!(factor_names.contains(&"decision_confidence"));
    assert!(factor_names.contains(&"risk_tier"));
    assert!(factor_names.contains(&"hard_gate_status"));
    assert!(factor_names.contains(&"test_stability"));

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn risk_breakdown_persisted_when_blocked_by_gates() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir = unique_temp_dir("risk_blocked");

    // Default gate signals are all failing → run exits with 3 (Blocked)
    let output = Command::new(bin)
        .arg(&out_dir)
        .output()
        .expect("failed to execute orchestrator");

    assert_eq!(output.status.code(), Some(3));

    let report_raw =
        fs::read_to_string(out_dir.join("orchestrator_run_report.json")).expect("report missing");
    let report: RiskReportView = from_str(&report_raw).expect("failed to parse report");

    assert_eq!(report.terminal_state.as_deref(), Some("blocked"));

    let breakdown = report
        .pr_risk_breakdown
        .expect("pr_risk_breakdown must be present even when blocked");
    assert!(breakdown.total_score > 0);
    assert!(!breakdown.eligible_for_auto_merge);

    let gate_factor = breakdown
        .factors
        .iter()
        .find(|f| f.name == "gate_quality")
        .expect("gate_quality factor required");
    assert!(gate_factor.score > 0);

    let _ = fs::remove_dir_all(out_dir);
}

// --- E2E: eligible path with --auto-merge-on-eligible ---

#[test]
fn auto_merge_eligible_path_emits_reason_code() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir = unique_temp_dir("risk_eligible");

    let output = Command::new(bin)
        .arg(&out_dir)
        .arg("--policy-status")
        .arg("allow")
        .arg("--ci-status")
        .arg("success")
        .arg("--review-status")
        .arg("approved")
        .arg("--decision-contribution")
        .arg(
            "contributor_id=e2e,capability=validation,vote=proceed,confidence=100,weight=100",
        )
        .arg("--pr-risk-threshold")
        .arg("40")
        .arg("--auto-merge-on-eligible")
        .output()
        .expect("failed to execute orchestrator");

    assert!(
        output.status.success(),
        "expected exit 0, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let report_raw =
        fs::read_to_string(out_dir.join("orchestrator_run_report.json")).expect("report missing");
    let report: RiskReportView = from_str(&report_raw).expect("failed to parse report");

    assert_eq!(report.terminal_state.as_deref(), Some("done"));
    assert!(
        report
            .decision_rationale_codes
            .contains(&"PR_RISK_ELIGIBLE_FOR_AUTO_MERGE".to_string()),
        "expected PR_RISK_ELIGIBLE_FOR_AUTO_MERGE in rationale codes, got: {:?}",
        report.decision_rationale_codes
    );
    assert!(
        !report
            .blocked_reason_codes
            .contains(&"PR_RISK_ABOVE_THRESHOLD".to_string())
    );

    let breakdown = report.pr_risk_breakdown.expect("breakdown must be present");
    assert!(breakdown.eligible_for_auto_merge);

    let _ = fs::remove_dir_all(out_dir);
}

// --- E2E: blocked path above threshold with --auto-merge-on-eligible ---

#[test]
fn auto_merge_blocked_above_threshold_emits_reason_code() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir = unique_temp_dir("risk_above");

    // Use --simulate-blocked to force terminal_state=Blocked with all gates passing.
    // With terminal_state=Blocked, risk_tier_score=8. With threshold=0, 8 > 0 → not eligible.
    let _output = Command::new(bin)
        .arg(&out_dir)
        .arg("--policy-status")
        .arg("allow")
        .arg("--ci-status")
        .arg("success")
        .arg("--review-status")
        .arg("approved")
        .arg("--simulate-blocked")
        .arg("--pr-risk-threshold")
        .arg("0") // threshold=0: any score > 0 blocks auto-merge
        .arg("--auto-merge-on-eligible")
        .output()
        .expect("failed to execute orchestrator");

    let report_raw =
        fs::read_to_string(out_dir.join("orchestrator_run_report.json")).expect("report missing");
    let report: RiskReportView = from_str(&report_raw).expect("failed to parse report");

    assert!(
        report
            .blocked_reason_codes
            .contains(&"PR_RISK_ABOVE_THRESHOLD".to_string()),
        "expected PR_RISK_ABOVE_THRESHOLD in blocked_reason_codes, got: {:?}",
        report.blocked_reason_codes
    );
    assert_eq!(
        report.terminal_state.as_deref(),
        Some("blocked"),
        "expected terminal state blocked when risk above threshold with auto-merge flag set"
    );

    let breakdown = report.pr_risk_breakdown.expect("breakdown must be present");
    assert!(!breakdown.eligible_for_auto_merge);
    assert!(breakdown.total_score > 0);

    let _ = fs::remove_dir_all(out_dir);
}
