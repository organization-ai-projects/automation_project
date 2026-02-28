use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use common_json::from_str;

#[derive(Debug, serde::Deserialize)]
struct ProvenanceReportView {
    terminal_state: Option<String>,
    blocked_reason_codes: Vec<String>,
    provenance_records: Vec<ProvenanceRecordView>,
    provenance_schema_version: String,
}

#[derive(Debug, serde::Deserialize)]
struct ProvenanceRecordView {
    id: String,
    event_type: String,
    parent_ids: Vec<String>,
    reason_codes: Vec<String>,
}

fn unique_temp_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let pid = std::process::id();
    let dir = std::env::temp_dir().join(format!(
        "autonomy_orchestrator_provenance_{name}_{pid}_{nanos}"
    ));
    fs::create_dir_all(&dir).expect("failed to create temp dir");
    dir
}

#[test]
fn full_run_yields_connected_non_empty_provenance_chain() {
    let bin = env!("CARGO_BIN_EXE_autonomy_orchestrator_ai");
    let out_dir = unique_temp_dir("e2e_chain");

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
            "contributor_id=e2e_prov,capability=validation,vote=proceed,confidence=100,weight=100",
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
    let report: ProvenanceReportView =
        from_str(&report_raw).expect("failed to deserialize run report");

    assert_eq!(report.terminal_state.as_deref(), Some("done"));
    assert_eq!(report.provenance_schema_version, "1");
    assert!(
        !report.provenance_records.is_empty(),
        "provenance chain must be non-empty for a full run"
    );

    // All nodes carry PROVENANCE_NODE_RECORDED reason code.
    for record in &report.provenance_records {
        assert!(
            record
                .reason_codes
                .contains(&"PROVENANCE_NODE_RECORDED".to_string()),
            "node '{}' is missing PROVENANCE_NODE_RECORDED reason code",
            record.id
        );
    }

    // Chain is connected: all parent_ids reference existing node ids.
    let ids: HashSet<&str> = report
        .provenance_records
        .iter()
        .map(|r| r.id.as_str())
        .collect();
    for record in &report.provenance_records {
        for parent_id in &record.parent_ids {
            assert!(
                ids.contains(parent_id.as_str()),
                "node '{}' references missing parent '{}'",
                record.id,
                parent_id
            );
        }
    }

    // Required event types are present.
    let event_types: Vec<&str> = report
        .provenance_records
        .iter()
        .map(|r| r.event_type.as_str())
        .collect();
    assert!(
        event_types.iter().any(|t| t.starts_with("stage_transition:")),
        "expected stage transition nodes in provenance chain"
    );
    assert!(
        event_types.contains(&"gate_evaluation"),
        "expected gate_evaluation node in provenance chain"
    );
    assert!(
        event_types.contains(&"final_decision"),
        "expected final_decision node in provenance chain"
    );
    assert!(
        event_types.contains(&"terminal_state"),
        "expected terminal_state node in provenance chain"
    );

    assert!(
        !report
            .blocked_reason_codes
            .contains(&"PROVENANCE_CHAIN_INCOMPLETE".to_string()),
        "PROVENANCE_CHAIN_INCOMPLETE must not be present in a valid run"
    );

    let _ = fs::remove_dir_all(out_dir);
}
