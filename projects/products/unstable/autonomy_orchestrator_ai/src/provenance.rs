// projects/products/unstable/autonomy_orchestrator_ai/src/provenance.rs
use crate::domain::ProvenanceRecord;
use std::collections::HashSet;

pub const SCHEMA_VERSION: &str = "1";

pub fn record_node(
    records: &mut Vec<ProvenanceRecord>,
    id: String,
    event_type: String,
    parent_ids: Vec<String>,
    reason_codes: Vec<String>,
    artifact_refs: Vec<String>,
    timestamp_unix_secs: u64,
) {
    let mut codes = reason_codes;
    codes.push("PROVENANCE_NODE_RECORDED".to_string());
    records.push(ProvenanceRecord {
        id,
        event_type,
        parent_ids,
        reason_codes: codes,
        artifact_refs,
        timestamp_unix_secs,
    });
}

pub fn validate_chain_completeness(records: &[ProvenanceRecord]) -> Result<(), String> {
    let ids: HashSet<&str> = records.iter().map(|r| r.id.as_str()).collect();
    for record in records {
        for parent_id in &record.parent_ids {
            if !ids.contains(parent_id.as_str()) {
                return Err(format!(
                    "PROVENANCE_CHAIN_INCOMPLETE: node '{}' references missing parent '{}'",
                    record.id, parent_id
                ));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_record(
        id: &str,
        parent_ids: Vec<&str>,
        timestamp: u64,
    ) -> ProvenanceRecord {
        ProvenanceRecord {
            id: id.to_string(),
            event_type: "test_event".to_string(),
            parent_ids: parent_ids.into_iter().map(|s| s.to_string()).collect(),
            reason_codes: vec!["PROVENANCE_NODE_RECORDED".to_string()],
            artifact_refs: Vec::new(),
            timestamp_unix_secs: timestamp,
        }
    }

    #[test]
    fn empty_chain_is_valid() {
        assert!(validate_chain_completeness(&[]).is_ok());
    }

    #[test]
    fn single_root_node_is_valid() {
        let records = vec![make_record("root", vec![], 1)];
        assert!(validate_chain_completeness(&records).is_ok());
    }

    #[test]
    fn chain_with_valid_parent_is_valid() {
        let records = vec![
            make_record("root", vec![], 1),
            make_record("child", vec!["root"], 2),
            make_record("grandchild", vec!["child"], 3),
        ];
        assert!(validate_chain_completeness(&records).is_ok());
    }

    #[test]
    fn chain_with_missing_parent_returns_error() {
        let records = vec![
            make_record("root", vec![], 1),
            make_record("child", vec!["missing_parent"], 2),
        ];
        let result = validate_chain_completeness(&records);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("PROVENANCE_CHAIN_INCOMPLETE"));
        assert!(err.contains("missing_parent"));
    }

    #[test]
    fn record_node_appends_provenance_node_recorded_reason_code() {
        let mut records = Vec::new();
        record_node(
            &mut records,
            "node_1".to_string(),
            "run_start".to_string(),
            vec![],
            vec!["MY_CODE".to_string()],
            vec![],
            100,
        );
        assert_eq!(records.len(), 1);
        assert!(
            records[0]
                .reason_codes
                .contains(&"PROVENANCE_NODE_RECORDED".to_string())
        );
        assert!(records[0].reason_codes.contains(&"MY_CODE".to_string()));
    }
}
