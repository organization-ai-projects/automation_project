//! projects/products/unstable/neurosymbolic_moe/backend/src/router/tests/router_trait.rs
use std::collections::HashMap;

use crate::expert_registry::ExpertRegistry;
use crate::moe_core::{ExpertId, MoeError, Task, TaskType};
use crate::router::{Router, RoutingDecision, RoutingStrategy};

struct StubRouter;

impl Router for StubRouter {
    fn route(&self, task: &Task, _registry: &ExpertRegistry) -> Result<RoutingDecision, MoeError> {
        Ok(RoutingDecision {
            task_id: task.id().clone(),
            selected_experts: vec![ExpertId::new("stub")],
            scores: HashMap::from([(ExpertId::new("stub"), 1.0)]),
            strategy: RoutingStrategy::SingleExpert,
            explanation: "stub router".to_string(),
        })
    }
}

#[test]
fn router_trait_can_be_invoked_through_dyn_dispatch() {
    let router: Box<dyn Router> = Box::new(StubRouter);
    let registry = ExpertRegistry::new();
    let task = Task::new("t1", TaskType::Planning, "plan");
    let result = router.route(&task, &registry);
    assert!(result.is_ok());
    let decision = result.expect("stub route must succeed");
    assert_eq!(decision.selected_experts.len(), 1);
    assert_eq!(decision.selected_experts[0].as_str(), "stub");
}
