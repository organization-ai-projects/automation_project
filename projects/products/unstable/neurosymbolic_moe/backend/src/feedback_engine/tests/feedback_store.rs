use crate::feedback_engine::{FeedbackEntry, FeedbackStore, FeedbackType};
use crate::moe_core::{ExpertId, TaskId};

fn make_feedback(id: &str, task: &str, expert: &str, feedback_type: FeedbackType) -> FeedbackEntry {
    FeedbackEntry {
        id: id.to_string(),
        task_id: TaskId::new(task),
        expert_id: ExpertId::new(expert),
        feedback_type,
        score: Some(0.8),
        comment: "good".to_string(),
        created_at: 1,
    }
}

#[test]
fn add_and_count() {
    let mut store = FeedbackStore::new();
    store.add(make_feedback("f1", "t1", "e1", FeedbackType::Positive));
    store.add(make_feedback("f2", "t1", "e1", FeedbackType::Negative));
    assert_eq!(store.count(), 2);
}

#[test]
fn get_by_task() {
    let mut store = FeedbackStore::new();
    store.add(make_feedback("f1", "t1", "e1", FeedbackType::Positive));
    store.add(make_feedback("f2", "t2", "e1", FeedbackType::Positive));
    assert_eq!(store.get_by_task(&TaskId::new("t1")).len(), 1);
}

#[test]
fn get_by_expert() {
    let mut store = FeedbackStore::new();
    store.add(make_feedback("f1", "t1", "e1", FeedbackType::Positive));
    store.add(make_feedback("f2", "t1", "e2", FeedbackType::Positive));
    assert_eq!(store.get_by_expert(&ExpertId::new("e1")).len(), 1);
}

#[test]
fn get_by_type() {
    let mut store = FeedbackStore::new();
    store.add(make_feedback("f1", "t1", "e1", FeedbackType::Positive));
    store.add(make_feedback("f2", "t1", "e1", FeedbackType::Negative));
    assert_eq!(store.get_by_type(&FeedbackType::Positive).len(), 1);
}

#[test]
fn average_score_for_expert() {
    let mut store = FeedbackStore::new();
    let mut first = make_feedback("f1", "t1", "e1", FeedbackType::Positive);
    first.score = Some(0.8);
    let mut second = make_feedback("f2", "t2", "e1", FeedbackType::Positive);
    second.score = Some(0.6);
    store.add(first);
    store.add(second);
    let avg = store
        .average_score_for_expert(&ExpertId::new("e1"))
        .expect("average should exist for expert with scored feedback");
    assert!((avg - 0.7).abs() < f64::EPSILON);
}
