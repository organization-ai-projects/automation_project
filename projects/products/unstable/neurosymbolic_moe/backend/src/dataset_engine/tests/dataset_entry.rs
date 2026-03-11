use crate::dataset_engine::{Correction, DatasetEntry, Outcome};
use crate::moe_core::{ExpertId, TaskId};
use std::collections::HashMap;

#[test]
fn dataset_entry_fields_round_trip() {
    let mut metadata = HashMap::new();
    metadata.insert("k".to_string(), "v".to_string());

    let entry = DatasetEntry {
        id: "d1".to_string(),
        task_id: TaskId::new("t1"),
        expert_id: ExpertId::new("e1"),
        input: "input".to_string(),
        output: "output".to_string(),
        outcome: Outcome::Partial,
        score: Some(0.42),
        tags: vec!["tag-a".to_string(), "tag-b".to_string()],
        created_at: 123,
        metadata,
    };

    assert_eq!(entry.id, "d1");
    assert_eq!(entry.task_id.as_str(), "t1");
    assert_eq!(entry.expert_id.as_str(), "e1");
    assert!(matches!(entry.outcome, Outcome::Partial));
    assert_eq!(entry.score, Some(0.42));
    assert_eq!(entry.tags.len(), 2);
    assert_eq!(entry.metadata.get("k").map(String::as_str), Some("v"));
}

#[test]
fn correction_fields_round_trip() {
    let correction = Correction {
        entry_id: "d1".to_string(),
        corrected_output: "new output".to_string(),
        reason: "manual review".to_string(),
        corrected_at: 456,
    };

    assert_eq!(correction.entry_id, "d1");
    assert_eq!(correction.corrected_output, "new output");
    assert_eq!(correction.reason, "manual review");
    assert_eq!(correction.corrected_at, 456);
}
