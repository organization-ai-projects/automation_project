use crate::feedback_engine::{FeedbackEntry, FeedbackType};
use crate::moe_core::{ExpertId, TaskId};
use protocol::ProtocolId;

fn protocol_id(_byte: u8) -> ProtocolId {
    ProtocolId::default()
}

fn task_id(byte: u8) -> TaskId {
    TaskId::from_protocol_id(protocol_id(byte))
}

fn expert_id(byte: u8) -> ExpertId {
    ExpertId::from_protocol_id(protocol_id(byte))
}

#[test]
fn feedback_entry_fields_round_trip() {
    let entry = FeedbackEntry {
        id: protocol_id(1),
        task_id: task_id(2),
        expert_id: expert_id(3),
        feedback_type: FeedbackType::Suggestion,
        score: Some(0.5),
        comment: "improve".to_string(),
        created_at: 10,
    };
    assert_eq!(entry.id, protocol_id(1));
    assert_eq!(entry.task_id, task_id(2));
    assert_eq!(entry.expert_id, expert_id(3));
    assert!(matches!(entry.feedback_type, FeedbackType::Suggestion));
}
