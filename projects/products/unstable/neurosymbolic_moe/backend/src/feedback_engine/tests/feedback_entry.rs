use crate::feedback_engine::{FeedbackEntry, FeedbackType};
use crate::moe_core::{ExpertId, TaskId};

#[test]
fn feedback_entry_fields_round_trip() {
    let entry = FeedbackEntry {
        id: "fb1".to_string(),
        task_id: TaskId::new("t1"),
        expert_id: ExpertId::new("e1"),
        feedback_type: FeedbackType::Suggestion,
        score: Some(0.5),
        comment: "improve".to_string(),
        created_at: 10,
    };
    assert_eq!(entry.id, "fb1");
    assert_eq!(entry.task_id.as_str(), "t1");
    assert_eq!(entry.expert_id.as_str(), "e1");
    assert!(matches!(entry.feedback_type, FeedbackType::Suggestion));
}
