//! projects/products/unstable/neurosymbolic_moe/backend/src/router/tests/routing_decision.rs
use std::collections::HashMap;

use crate::moe_core::{ExpertId, TaskId};
use crate::router::{RoutingDecision, RoutingStrategy};
use protocol::ProtocolId;
use std::str::FromStr;

fn protocol_id(byte: u8) -> ProtocolId {
    ProtocolId::from_str(&format!("{:032x}", byte.max(1)))
        .expect("test protocol id should be valid fixed hex")
}

fn task_id(byte: u8) -> TaskId {
    TaskId::from_protocol_id(protocol_id(byte))
}

fn expert_id(byte: u8) -> ExpertId {
    ExpertId::from_protocol_id(protocol_id(byte))
}

#[test]
fn routing_decision_carries_selected_experts_and_scores() {
    let task_id = task_id(1);
    let expert_a = expert_id(1);
    let expert_b = expert_id(2);
    let mut scores = HashMap::new();
    scores.insert(expert_a.clone(), 0.8);
    scores.insert(expert_b.clone(), 0.6);

    let decision = RoutingDecision {
        task_id: task_id.clone(),
        selected_experts: vec![expert_a.clone(), expert_b.clone()],
        scores,
        strategy: RoutingStrategy::MultiExpert,
        explanation: "top two experts selected".to_string(),
    };

    assert_eq!(decision.task_id, task_id);
    assert_eq!(decision.selected_experts.len(), 2);
    assert!(matches!(decision.strategy, RoutingStrategy::MultiExpert));
    assert!(decision.scores.contains_key(&expert_a));
}
