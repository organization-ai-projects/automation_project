//! projects/products/unstable/neurosymbolic_moe/backend/src/router/tests/routing_trace.rs
use std::collections::HashMap;

use crate::router::{RoutingDecision, RoutingStrategy, RoutingTrace};

#[test]
fn routing_trace_from_decision_uses_deterministic_timestamp() {
    let task_id = crate::tests::helpers::task_id(1);
    let expert_id = crate::tests::helpers::expert_id(1);
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
