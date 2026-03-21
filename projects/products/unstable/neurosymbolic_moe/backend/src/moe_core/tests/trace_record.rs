use crate::moe_core::{self, TracePhase, TraceRecord};
use std::collections::HashMap;

fn task_id(byte: u8) -> moe_core::TaskId {
    crate::tests::helpers::task_id(byte)
}

fn expert_id(byte: u8) -> moe_core::ExpertId {
    crate::tests::helpers::expert_id(byte)
}

#[test]
fn trace_record_is_constructible() {
    let record = TraceRecord {
        trace_id: "tr-2".to_string(),
        task_id: task_id(1),
        timestamp: 456,
        expert_id: Some(expert_id(2)),
        phase: TracePhase::Validation,
        detail: "ok".to_string(),
        metadata: HashMap::new(),
    };
    assert_eq!(record.trace_id, "tr-2");
    assert_eq!(record.task_id, task_id(1));
    assert!(matches!(record.phase, TracePhase::Validation));
}
