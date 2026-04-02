use crate::dataset_engine::{Correction, DatasetEntry, Outcome};
use crate::moe_core::{ExpertId, TaskId};
use protocol::ProtocolId;
use std::collections::HashMap;
use std::str::FromStr;

fn protocol_id(byte: u8) -> ProtocolId {
    ProtocolId::from_str(&format!("{:032x}", byte.max(1)))
        .expect("test protocol id should be valid fixed hex")
}

fn task_id(byte: u8) -> TaskId {
    TaskId::from_protocol_id(protocol_id(byte))
}

fn expert_id(byte: u8) -> ExpertId {
    ExpertId::from_protocol_id(protocol_id(byte))
}

#[test]
fn dataset_entry_fields_round_trip() {
    let mut metadata = HashMap::new();
    metadata.insert("k".to_string(), "v".to_string());

    let entry = DatasetEntry {
        id: protocol_id(1),
        task_id: task_id(2),
        expert_id: expert_id(3),
        input: "input".to_string(),
        output: "output".to_string(),
        outcome: Outcome::Partial,
        score: Some(0.42),
        tags: vec!["tag-a".to_string(), "tag-b".to_string()],
        created_at: 123,
        metadata,
    };

    assert_eq!(entry.id, protocol_id(1));
    assert_eq!(entry.task_id, task_id(2));
    assert_eq!(entry.expert_id, expert_id(3));
    assert!(matches!(entry.outcome, Outcome::Partial));
    assert_eq!(entry.score, Some(0.42));
    assert_eq!(entry.tags.len(), 2);
    assert_eq!(entry.metadata.get("k").map(String::as_str), Some("v"));
}

#[test]
fn correction_fields_round_trip() {
    let correction = Correction {
        entry_id: protocol_id(4),
        corrected_output: "new output".to_string(),
        reason: "manual review".to_string(),
        corrected_at: 456,
    };

    assert_eq!(correction.entry_id, protocol_id(4));
    assert_eq!(correction.corrected_output, "new output");
    assert_eq!(correction.reason, "manual review");
    assert_eq!(correction.corrected_at, 456);
}
