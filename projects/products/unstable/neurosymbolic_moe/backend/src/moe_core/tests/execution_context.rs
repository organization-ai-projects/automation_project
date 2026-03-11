use crate::moe_core::{ExecutionContext, TaskId};

#[test]
fn new_creates_empty_context() {
    let ctx = ExecutionContext::new(TaskId::new("t1"));
    assert_eq!(ctx.task_id.as_str(), "t1");
    assert!(ctx.retrieved_context.is_empty());
    assert!(ctx.memory_entries.is_empty());
    assert!(ctx.buffer_data.is_empty());
    assert!(ctx.parameters.is_empty());
}

#[test]
fn with_retrieved_context_sets_context() {
    let ctx = ExecutionContext::new(TaskId::new("t1"))
        .with_retrieved_context(vec!["ctx1".to_string(), "ctx2".to_string()]);
    assert_eq!(ctx.retrieved_context.len(), 2);
    assert_eq!(ctx.retrieved_context[0], "ctx1");
}

#[test]
fn with_memory_entries_sets_entries() {
    let ctx =
        ExecutionContext::new(TaskId::new("t1")).with_memory_entries(vec!["mem1".to_string()]);
    assert_eq!(ctx.memory_entries, vec!["mem1"]);
}

#[test]
fn with_buffer_data_sets_data() {
    let ctx = ExecutionContext::new(TaskId::new("t1")).with_buffer_data(vec!["buf1".to_string()]);
    assert_eq!(ctx.buffer_data, vec!["buf1"]);
}

#[test]
fn with_parameter_adds_parameter() {
    let ctx = ExecutionContext::new(TaskId::new("t1"))
        .with_parameter("k", "v")
        .with_parameter("k2", "v2");
    assert_eq!(ctx.parameters.get("k").map(String::as_str), Some("v"));
    assert_eq!(ctx.parameters.get("k2").map(String::as_str), Some("v2"));
}
