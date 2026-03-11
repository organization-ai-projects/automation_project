use crate::moe_core::{ExpertId, TaskId, TracePhase, TraceRecord};
use std::collections::HashMap;

#[test]
fn trace_record_is_constructible() {
    let record = TraceRecord {
        trace_id: "tr-2".to_string(),
        task_id: TaskId::new("t2"),
        timestamp: 456,
        expert_id: Some(ExpertId::new("e2")),
        phase: TracePhase::Validation,
        detail: "ok".to_string(),
        metadata: HashMap::new(),
    };
    assert_eq!(record.trace_id, "tr-2");
    assert_eq!(record.task_id.as_str(), "t2");
    assert!(matches!(record.phase, TracePhase::Validation));
}
