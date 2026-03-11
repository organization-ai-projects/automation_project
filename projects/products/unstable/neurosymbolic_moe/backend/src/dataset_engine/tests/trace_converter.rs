use crate::dataset_engine::{Outcome, TraceConverter};
use crate::moe_core::{ExpertId, TaskId, TracePhase, TraceRecord};
use std::collections::HashMap;

fn make_trace(
    trace_id: &str,
    task_id: &str,
    timestamp: u64,
    expert_id: Option<&str>,
    phase: TracePhase,
) -> TraceRecord {
    TraceRecord {
        trace_id: trace_id.to_string(),
        task_id: TaskId::new(task_id),
        timestamp,
        expert_id: expert_id.map(ExpertId::new),
        phase,
        detail: "detail".to_string(),
        metadata: HashMap::new(),
    }
}

#[test]
fn convert_populates_dataset_entry_from_traces() {
    let converter = TraceConverter::new();
    let traces = vec![
        make_trace("tr1", "task-1", 10, Some("expert-a"), TracePhase::Routing),
        make_trace(
            "tr2",
            "task-1",
            25,
            Some("expert-a"),
            TracePhase::ExpertExecution,
        ),
    ];

    let dataset = converter.convert(&traces, "input", "output", Outcome::Success);

    assert_eq!(dataset.task_id.as_str(), "task-1");
    assert_eq!(dataset.expert_id.as_str(), "expert-a");
    assert_eq!(dataset.created_at, 25);
    assert_eq!(
        dataset.metadata.get("trace_count").map(String::as_str),
        Some("2")
    );
    assert_eq!(
        dataset.metadata.get("first_trace_id").map(String::as_str),
        Some("tr1")
    );
    assert_eq!(
        dataset.metadata.get("last_trace_id").map(String::as_str),
        Some("tr2")
    );
}

#[test]
fn convert_uses_unknown_defaults_on_empty_traces() {
    let converter = TraceConverter::new();
    let dataset = converter.convert(&[], "input", "output", Outcome::Unknown);
    assert_eq!(dataset.task_id.as_str(), "unknown");
    assert_eq!(dataset.expert_id.as_str(), "unknown");
    assert_eq!(dataset.created_at, 0);
}

#[test]
fn extract_tags_is_unique_and_sorted() {
    let converter = TraceConverter::new();
    let traces = vec![
        make_trace("t1", "task", 1, None, TracePhase::Routing),
        make_trace("t2", "task", 2, None, TracePhase::Routing),
        make_trace("t3", "task", 3, None, TracePhase::Retrieval),
    ];
    let tags = converter.extract_tags(&traces);
    assert_eq!(
        tags,
        vec!["phase:Retrieval".to_string(), "phase:Routing".to_string()]
    );
}
