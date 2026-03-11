//! projects/products/unstable/neurosymbolic_moe/backend/src/router/tests/routing_decision.rs
use std::collections::HashMap;

use crate::moe_core::{ExpertId, TaskId};
use crate::router::{RoutingDecision, RoutingStrategy};

#[test]
fn routing_decision_carries_selected_experts_and_scores() {
    let mut scores = HashMap::new();
    scores.insert(ExpertId::new("e1"), 0.8);
    scores.insert(ExpertId::new("e2"), 0.6);

    let decision = RoutingDecision {
        task_id: TaskId::new("task-1"),
        selected_experts: vec![ExpertId::new("e1"), ExpertId::new("e2")],
        scores,
        strategy: RoutingStrategy::MultiExpert,
        explanation: "top two experts selected".to_string(),
    };

    assert_eq!(decision.task_id.as_str(), "task-1");
    assert_eq!(decision.selected_experts.len(), 2);
    assert!(matches!(decision.strategy, RoutingStrategy::MultiExpert));
    assert!(decision.scores.contains_key(&ExpertId::new("e1")));
}
