//! projects/products/unstable/neurosymbolic_moe/backend/src/retrieval_engine/tests/context_assembler.rs
use crate::moe_core::{Task, TaskType};
use crate::retrieval_engine::{ContextAssembler, RetrievalResult};

#[test]
fn assemble_respects_context_budget() {
    let assembler = ContextAssembler::new(12);
    let results = vec![
        RetrievalResult::new("c1", "abcdef", 0.9, "doc"),
        RetrievalResult::new("c2", "ghijkl", 0.8, "doc"),
        RetrievalResult::new("c3", "mnopqr", 0.7, "doc"),
    ];
    let assembled = assembler.assemble(&results);
    assert!(!assembled.is_empty());
    let total: usize = assembled.iter().map(String::len).sum();
    assert!(total <= 12);
}

#[test]
fn assemble_for_task_prepends_header() {
    let assembler = ContextAssembler::new(60);
    let task = Task::new("task-ctx", TaskType::Retrieval, "find context");
    let results = vec![RetrievalResult::new("c1", "retrieved block", 0.9, "doc")];
    let assembled = assembler.assemble_for_task(&results, &task);
    assert!(!assembled.is_empty());
    assert!(assembled[0].contains("[task:task-ctx]"));
}
