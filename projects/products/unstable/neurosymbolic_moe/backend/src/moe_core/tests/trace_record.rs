use crate::moe_core::{self, TracePhase, TraceRecord};
use protocol::ProtocolId;
use std::collections::HashMap;

fn task_id(_byte: u8) -> moe_core::TaskId {
    moe_core::TaskId::from_protocol_id(ProtocolId::default())
}

fn expert_id(_byte: u8) -> moe_core::ExpertId {
    moe_core::ExpertId::from_protocol_id(ProtocolId::default())
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
