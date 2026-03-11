use crate::moe_core::{Task, TaskPriority, TaskType};

#[test]
fn new_creates_task_with_correct_fields() {
    let task = Task::new("t1", TaskType::CodeGeneration, "write code");
    assert_eq!(task.id().as_str(), "t1");
    assert_eq!(task.input(), "write code");
    assert!(task.context().is_none());
    assert!(matches!(task.priority(), TaskPriority::Normal));
    assert!(task.metadata.is_empty());
}

#[test]
fn with_context_sets_context() {
    let task = Task::new("t2", TaskType::CodeAnalysis, "analyze").with_context("extra ctx");
    assert_eq!(task.context(), Some("extra ctx"));
}

#[test]
fn with_priority_sets_priority() {
    let task =
        Task::new("t3", TaskType::Retrieval, "retrieve").with_priority(TaskPriority::Critical);
    assert!(matches!(task.priority(), TaskPriority::Critical));
}

#[test]
fn with_metadata_adds_entry() {
    let task = Task::new("t4", TaskType::Validation, "validate")
        .with_metadata("key", "value")
        .with_metadata("k2", "v2");
    assert_eq!(task.metadata.get("key").map(String::as_str), Some("value"));
    assert_eq!(task.metadata.get("k2").map(String::as_str), Some("v2"));
}
