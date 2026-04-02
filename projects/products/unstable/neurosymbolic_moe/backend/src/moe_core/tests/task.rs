//! projects/products/unstable/neurosymbolic_moe/backend/src/moe_core/tests/task.rs
use crate::moe_core::{self, Task, TaskPriority, TaskType};
use protocol::ProtocolId;
use std::str::FromStr;

fn protocol_id(byte: u8) -> ProtocolId {
    ProtocolId::from_str(&format!("{:032x}", byte.max(1)))
        .expect("test protocol id should be valid fixed hex")
}

fn task_id(byte: u8) -> moe_core::TaskId {
    moe_core::TaskId::from_protocol_id(protocol_id(byte))
}

#[test]
fn new_creates_task_with_correct_fields() {
    let task = Task::new_with_id(protocol_id(1), TaskType::CodeGeneration, "write code");
    assert_eq!(task.id(), &task_id(1));
    assert_eq!(task.input(), "write code");
    assert!(task.context().is_none());
    assert!(matches!(task.priority(), TaskPriority::Normal));
    assert!(task.metadata.is_empty());
}

#[test]
fn with_context_sets_context() {
    let task = Task::new_with_id(protocol_id(2), TaskType::CodeAnalysis, "analyze")
        .with_context("extra ctx");
    assert_eq!(task.context(), Some("extra ctx"));
}

#[test]
fn with_priority_sets_priority() {
    let task = Task::new_with_id(protocol_id(3), TaskType::Retrieval, "retrieve")
        .with_priority(TaskPriority::Critical);
    assert!(matches!(task.priority(), TaskPriority::Critical));
}

#[test]
fn with_metadata_adds_entry() {
    let task = Task::new_with_id(protocol_id(4), TaskType::Validation, "validate")
        .with_metadata("key", "value")
        .with_metadata("k2", "v2");
    assert_eq!(task.metadata.get("key").map(String::as_str), Some("value"));
    assert_eq!(task.metadata.get("k2").map(String::as_str), Some("v2"));
}
