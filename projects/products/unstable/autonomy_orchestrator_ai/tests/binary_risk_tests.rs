use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use common_json::from_str;

#[derive(Debug, serde::Deserialize)]
struct RiskReportView {
    terminal_state: Option<String>,
    blocked_reason_codes: Vec<String>,
    risk_tier: Option<String>,
    risk_signals: Vec<RiskSignalView>,
}

#[derive(Debug, serde::Deserialize)]
struct RiskSignalView {
    code: String,
    source: String,
    value: String,
}

fn unique_temp_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let pid = std::process::id();
    let dir =
        std::env::temp_dir().join(format!("autonomy_orchestrator_risk_{name}_{pid}_{nanos}"));
    fs::create_dir_all(&dir).expect("failed to create temp dir");
    dir
}

#[test]
fn risk_high_blocked_by_default() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir = unique_temp_dir("high_blocked");

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
            "contributor_id=risk_test,capability=validation,vote=proceed,confidence=100,weight=100",
        )
        .arg("--risk-tier-override")
        .arg("high")
        .output()
        .expect("failed to execute orchestrator");

    assert_eq!(
        run.status.code(),
        Some(3),
        "expected exit code 3 (Blocked) but got {:?}\nstdout: {}\nstderr: {}",
        run.status.code(),
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );

    let report_path = out_dir.join("orchestrator_run_report.json");
    let report_raw = fs::read_to_string(&report_path).expect("failed to read run report");
    let report: RiskReportView = from_str(&report_raw).expect("failed to deserialize run report");

    assert_eq!(report.terminal_state.as_deref(), Some("blocked"));
    assert_eq!(report.risk_tier.as_deref(), Some("high"));
    assert!(
        report
            .blocked_reason_codes
            .contains(&"RISK_TIER_HIGH_BLOCKED".to_string()),
        "expected RISK_TIER_HIGH_BLOCKED in {:?}",
        report.blocked_reason_codes
    );
    assert!(
        report
            .risk_signals
            .iter()
            .any(|s| s.code == "RISK_TIER_OVERRIDE_APPLIED"),
        "expected RISK_TIER_OVERRIDE_APPLIED signal"
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn risk_high_allowed_with_override_flag() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir = unique_temp_dir("high_allowed");

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
            "contributor_id=risk_test,capability=validation,vote=proceed,confidence=100,weight=100",
        )
        .arg("--risk-tier-override")
        .arg("high")
        .arg("--risk-allow-high")
        .output()
        .expect("failed to execute orchestrator");

    assert!(
        run.status.success(),
        "expected exit 0 (Done) but got {:?}\nstdout: {}\nstderr: {}",
        run.status.code(),
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );

    let report_path = out_dir.join("orchestrator_run_report.json");
    let report_raw = fs::read_to_string(&report_path).expect("failed to read run report");
    let report: RiskReportView = from_str(&report_raw).expect("failed to deserialize run report");

    assert_eq!(report.terminal_state.as_deref(), Some("done"));
    assert_eq!(report.risk_tier.as_deref(), Some("high"));
    assert!(
        !report
            .blocked_reason_codes
            .contains(&"RISK_TIER_HIGH_BLOCKED".to_string()),
        "expected no RISK_TIER_HIGH_BLOCKED but found it in {:?}",
        report.blocked_reason_codes
    );

    let _ = fs::remove_dir_all(out_dir);
}

#[test]
fn risk_low_tier_override_does_not_block() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir = unique_temp_dir("low_override");

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
            "contributor_id=risk_test,capability=validation,vote=proceed,confidence=100,weight=100",
        )
        .arg("--risk-tier-override")
        .arg("low")
        .output()
        .expect("failed to execute orchestrator");

    assert!(
        run.status.success(),
        "expected success for low risk but got {:?}\nstdout: {}\nstderr: {}",
        run.status.code(),
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );

    let report_path = out_dir.join("orchestrator_run_report.json");
    let report_raw = fs::read_to_string(&report_path).expect("failed to read run report");
    let report: RiskReportView = from_str(&report_raw).expect("failed to deserialize run report");

    assert_eq!(report.terminal_state.as_deref(), Some("done"));
    assert_eq!(report.risk_tier.as_deref(), Some("low"));
    assert!(report.risk_signals.iter().any(|s| s.code == "RISK_TIER_OVERRIDE_APPLIED"));

    let _ = fs::remove_dir_all(out_dir);
}
