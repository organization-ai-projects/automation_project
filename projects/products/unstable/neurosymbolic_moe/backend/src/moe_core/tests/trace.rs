use crate::moe_core::{ExpertId, TaskId, TracePhase, TraceRecord};
use std::collections::HashMap;

#[test]
fn trace_record_fields_round_trip() {
    let record = TraceRecord {
        trace_id: "tr-1".to_string(),
        task_id: TaskId::new("t1"),
        timestamp: 123,
        expert_id: Some(ExpertId::new("e1")),
        phase: TracePhase::Routing,
        detail: "detail".to_string(),
        metadata: HashMap::new(),
    };
    assert_eq!(record.trace_id, "tr-1");
    assert_eq!(record.task_id.as_str(), "t1");
    assert!(record.expert_id.is_some());
    assert!(matches!(record.phase, TracePhase::Routing));
}
