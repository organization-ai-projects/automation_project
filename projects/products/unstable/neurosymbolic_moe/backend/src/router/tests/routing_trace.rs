//! projects/products/unstable/neurosymbolic_moe/backend/src/router/tests/routing_trace.rs
use std::collections::HashMap;

use crate::moe_core::{ExpertId, TaskId};
use crate::router::{RoutingDecision, RoutingStrategy, RoutingTrace};

#[test]
fn routing_trace_from_decision_uses_deterministic_timestamp() {
    let decision = RoutingDecision {
        task_id: TaskId::new("task-42"),
        selected_experts: vec![ExpertId::new("expert-a")],
        scores: HashMap::from([(ExpertId::new("expert-a"), 0.9)]),
        strategy: RoutingStrategy::SingleExpert,
        explanation: "best match".to_string(),
    };

    let trace = RoutingTrace::from_decision(&decision, 7);

    assert_eq!(trace.task_id.as_str(), "task-42");
    assert_eq!(trace.candidates_evaluated, 7);
    assert_eq!(trace.timestamp, 7);
    assert_eq!(trace.selected.len(), 1);
}
