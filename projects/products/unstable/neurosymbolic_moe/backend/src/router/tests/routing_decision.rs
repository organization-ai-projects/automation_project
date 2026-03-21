//! projects/products/unstable/neurosymbolic_moe/backend/src/router/tests/routing_decision.rs
use std::collections::HashMap;

use crate::router::{RoutingDecision, RoutingStrategy};

#[test]
fn routing_decision_carries_selected_experts_and_scores() {
    let task_id = crate::tests::helpers::task_id(1);
    let expert_a = crate::tests::helpers::expert_id(1);
    let expert_b = crate::tests::helpers::expert_id(2);
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
