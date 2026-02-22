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
}

#[derive(Debug, serde::Deserialize)]
struct GateDecisionView {
    gate: String,
    passed: bool,
    reason_code: Option<String>,
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

#[test]
fn default_gate_signals_fail_closed_with_reason_codes() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
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
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir = unique_temp_dir("gates_pass");

    let run = Command::new(bin)
        .arg(&out_dir)
        .arg("--policy-status")
        .arg("allow")
        .arg("--ci-status")
        .arg("success")
        .arg("--review-status")
        .arg("approved")
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

    let _ = fs::remove_dir_all(out_dir);
}
