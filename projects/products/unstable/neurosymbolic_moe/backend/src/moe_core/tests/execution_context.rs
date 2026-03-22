//! projects/products/unstable/neurosymbolic_moe/backend/src/moe_core/tests/execution_context.rs
use crate::moe_core::{ExecutionContext, TaskId};
use protocol::ProtocolId;
use std::str::FromStr;

fn task_id() -> TaskId {
    TaskId::from_protocol_id(
        ProtocolId::from_str("00000000000000000000000000000001")
            .expect("test protocol id should be valid fixed hex"),
    )
}

#[test]
fn new_creates_empty_context() {
    let ctx = ExecutionContext::new(task_id());
    assert_eq!(ctx.task_id, task_id());
    assert!(ctx.retrieved_context.is_empty());
    assert!(ctx.memory_entries.is_empty());
    assert!(ctx.buffer_data.is_empty());
    assert!(ctx.parameters.is_empty());
}

#[test]
fn with_retrieved_context_sets_context() {
    let ctx = ExecutionContext::new(task_id())
        .with_retrieved_context(vec!["ctx1".to_string(), "ctx2".to_string()]);
    assert_eq!(ctx.retrieved_context.len(), 2);
    assert_eq!(ctx.retrieved_context[0], "ctx1");
}

#[test]
fn with_memory_entries_sets_entries() {
    let ctx = ExecutionContext::new(task_id()).with_memory_entries(vec!["mem1".to_string()]);
    assert_eq!(ctx.memory_entries, vec!["mem1"]);
}

#[test]
fn with_buffer_data_sets_data() {
    let ctx = ExecutionContext::new(task_id()).with_buffer_data(vec!["buf1".to_string()]);
    assert_eq!(ctx.buffer_data, vec!["buf1"]);
}

#[test]
fn with_parameter_adds_parameter() {
    let ctx = ExecutionContext::new(task_id())
        .with_parameter("k", "v")
        .with_parameter("k2", "v2");
    assert_eq!(ctx.parameters.get("k").map(String::as_str), Some("v"));
    assert_eq!(ctx.parameters.get("k2").map(String::as_str), Some("v2"));
}
