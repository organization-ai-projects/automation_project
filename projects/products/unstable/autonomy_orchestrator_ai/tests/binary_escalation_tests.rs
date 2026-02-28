use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use common_json::from_str;

#[derive(Debug, serde::Deserialize)]
struct EscalationCaseView {
    id: String,
    trigger_code: String,
    severity: String,
    required_actions: Vec<String>,
    context_artifacts: Vec<String>,
}

fn unique_temp_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let pid = std::process::id();
    let dir =
        std::env::temp_dir().join(format!("autonomy_orchestrator_esc_{name}_{pid}_{nanos}"));
    fs::create_dir_all(&dir).expect("failed to create temp dir");
    dir
}

// ── E2E: exception path emits escalation ─────────────────────────────────────

#[test]
fn gate_violation_emits_escalation_queue_artifact() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir = unique_temp_dir("esc_gate");

    // Default run: all gates fail → should produce an escalation queue artifact
    let run = Command::new(bin)
        .arg(&out_dir)
        .output()
        .expect("failed to execute orchestrator");

    assert_eq!(run.status.code(), Some(3), "expected blocked exit code");

    let queue_path = out_dir.join("escalation_queue.json");
    assert!(
        queue_path.exists(),
        "escalation_queue.json must be written on exception path"
    );

    let raw = fs::read_to_string(&queue_path).expect("failed to read escalation queue");
    let cases: Vec<EscalationCaseView> = from_str(&raw).expect("failed to parse escalation queue");

    assert!(!cases.is_empty(), "escalation cases must be non-empty");

    let policy_case = cases
        .iter()
        .find(|c| c.trigger_code == "ESCALATION_TRIGGER_POLICY_BLOCK")
        .expect("expected ESCALATION_TRIGGER_POLICY_BLOCK case");

    assert_eq!(policy_case.severity, "sev2");
    assert!(policy_case.id.contains("ESCALATION_TRIGGER_POLICY_BLOCK"));
    assert!(!policy_case.required_actions.is_empty());
    assert!(!policy_case.context_artifacts.is_empty());

    let _ = fs::remove_dir_all(out_dir);
}

// ── E2E: normal path emits no escalation ─────────────────────────────────────

#[test]
fn normal_path_does_not_write_escalation_queue_artifact() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir = unique_temp_dir("esc_normal");

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
            "contributor_id=esc_test,capability=validation,vote=proceed,confidence=100,weight=100",
        )
        .output()
        .expect("failed to execute orchestrator");

    assert!(
        run.status.success(),
        "expected done exit code, stdout={} stderr={}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );

    let queue_path = out_dir.join("escalation_queue.json");
    assert!(
        !queue_path.exists(),
        "escalation_queue.json must NOT be written on normal done path"
    );

    let _ = fs::remove_dir_all(out_dir);
}

// ── Integration: escalation payload completeness ─────────────────────────────

#[test]
fn escalation_queue_artifact_fields_are_complete() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir = unique_temp_dir("esc_payload");

    let _ = Command::new(bin)
        .arg(&out_dir)
        .output()
        .expect("failed to execute orchestrator");

    let queue_path = out_dir.join("escalation_queue.json");
    let raw = fs::read_to_string(&queue_path).expect("escalation_queue.json must exist");
    let cases: Vec<EscalationCaseView> = from_str(&raw).expect("must parse escalation queue");

    for case in &cases {
        assert!(!case.id.is_empty(), "case id must not be empty");
        assert!(!case.trigger_code.is_empty(), "trigger_code must not be empty");
        assert!(
            matches!(case.severity.as_str(), "sev1" | "sev2" | "sev3"),
            "severity must be sev1, sev2, or sev3; got '{}'",
            case.severity
        );
        assert!(
            !case.required_actions.is_empty(),
            "required_actions must not be empty for trigger {}",
            case.trigger_code
        );
    }

    let _ = fs::remove_dir_all(out_dir);
}

// ── Integration: run_report persists escalation_cases ────────────────────────

#[test]
fn run_report_persists_escalation_cases_on_exception() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir = unique_temp_dir("esc_report_persist");

    let _ = Command::new(bin)
        .arg(&out_dir)
        .output()
        .expect("failed to execute orchestrator");

    let report_path = out_dir.join("orchestrator_run_report.json");
    let raw = fs::read_to_string(&report_path).expect("run report must exist");

    #[derive(Debug, serde::Deserialize)]
    struct ReportView {
        escalation_cases: Vec<EscalationCaseView>,
    }

    let report: ReportView = from_str(&raw).expect("must parse run report");
    assert!(
        !report.escalation_cases.is_empty(),
        "run_report.escalation_cases must be populated on exception path"
    );

    let _ = fs::remove_dir_all(out_dir);
}
