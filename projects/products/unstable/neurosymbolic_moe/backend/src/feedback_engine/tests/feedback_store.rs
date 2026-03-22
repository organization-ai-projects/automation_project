use crate::feedback_engine::{FeedbackEntry, FeedbackStore, FeedbackType};
use crate::moe_core::{ExpertId, TaskId};
use protocol::ProtocolId;
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

fn make_feedback(id: u8, task: u8, expert: u8, feedback_type: FeedbackType) -> FeedbackEntry {
    FeedbackEntry {
        id: protocol_id(id),
        task_id: task_id(task),
        expert_id: expert_id(expert),
        feedback_type,
        score: Some(0.8),
        comment: "good".to_string(),
        created_at: 1,
    }
}

#[test]
fn add_and_count() {
    let mut store = FeedbackStore::new();
    store.add(make_feedback(1, 2, 3, FeedbackType::Positive));
    store.add(make_feedback(4, 2, 3, FeedbackType::Negative));
    assert_eq!(store.count(), 2);
}

#[test]
fn get_by_task() {
    let mut store = FeedbackStore::new();
    store.add(make_feedback(1, 2, 3, FeedbackType::Positive));
    store.add(make_feedback(4, 5, 3, FeedbackType::Positive));
    assert_eq!(store.get_by_task(&task_id(2)).len(), 1);
}

#[test]
fn get_by_expert() {
    let mut store = FeedbackStore::new();
    store.add(make_feedback(1, 2, 3, FeedbackType::Positive));
    store.add(make_feedback(4, 2, 6, FeedbackType::Positive));
    assert_eq!(store.get_by_expert(&expert_id(3)).len(), 1);
}

#[test]
fn get_by_type() {
    let mut store = FeedbackStore::new();
    store.add(make_feedback(1, 2, 3, FeedbackType::Positive));
    store.add(make_feedback(4, 2, 3, FeedbackType::Negative));
    assert_eq!(store.get_by_type(&FeedbackType::Positive).len(), 1);
}

#[test]
fn average_score_for_expert() {
    let mut store = FeedbackStore::new();
    let mut first = make_feedback(1, 2, 3, FeedbackType::Positive);
    first.score = Some(0.8);
    let mut second = make_feedback(4, 5, 3, FeedbackType::Positive);
    second.score = Some(0.6);
    store.add(first);
    store.add(second);
    let avg = store
        .average_score_for_expert(&expert_id(3))
        .expect("average should exist for expert with scored feedback");
    assert!((avg - 0.7).abs() < f64::EPSILON);
}
