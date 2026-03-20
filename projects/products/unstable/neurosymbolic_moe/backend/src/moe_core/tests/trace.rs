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
fn trace_record_fields_round_trip() {
    let record = TraceRecord {
        trace_id: "tr-1".to_string(),
        task_id: task_id(1),
        timestamp: 123,
        expert_id: Some(expert_id(2)),
        phase: TracePhase::Routing,
        detail: "detail".to_string(),
        metadata: HashMap::new(),
    };
    assert_eq!(record.trace_id, "tr-1");
    assert_eq!(record.task_id, task_id(1));
    assert!(record.expert_id.is_some());
    assert!(matches!(record.phase, TracePhase::Routing));
}
