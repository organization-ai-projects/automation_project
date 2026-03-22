//! projects/products/unstable/neurosymbolic_moe/backend/src/router/tests/routing_trace.rs
use std::collections::HashMap;

use crate::router::{RoutingDecision, RoutingStrategy, RoutingTrace};
use protocol::ProtocolId;
use std::str::FromStr;

fn protocol_id(byte: u8) -> ProtocolId {
    ProtocolId::from_str(&format!("{:032x}", byte.max(1)))
        .expect("test protocol id should be valid fixed hex")
}

fn task_id(byte: u8) -> crate::moe_core::TaskId {
    crate::moe_core::TaskId::from_protocol_id(protocol_id(byte))
}

fn expert_id(byte: u8) -> crate::moe_core::ExpertId {
    crate::moe_core::ExpertId::from_protocol_id(protocol_id(byte))
}

#[test]
fn routing_trace_from_decision_uses_deterministic_timestamp() {
    let task_id = task_id(1);
    let expert_id = expert_id(1);
    let decision = RoutingDecision {
        task_id: task_id.clone(),
        selected_experts: vec![expert_id.clone()],
        scores: HashMap::from([(expert_id, 0.9)]),
        strategy: RoutingStrategy::SingleExpert,
        explanation: "best match".to_string(),
    };

    let trace = RoutingTrace::from_decision(&decision, 7);

    assert_eq!(trace.task_id, task_id);
    assert_eq!(trace.candidates_evaluated, 7);
    assert_eq!(trace.timestamp, 7);
    assert_eq!(trace.selected.len(), 1);
}
