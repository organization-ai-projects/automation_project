//! projects/products/unstable/neurosymbolic_moe/backend/src/router/tests/heuristic_router.rs
use std::collections::HashMap;

use crate::expert_registries::ExpertRegistry;
use crate::moe_core::{
    ExecutionContext, Expert, ExpertCapability, ExpertError, ExpertId, ExpertMetadata,
    ExpertOutput, ExpertStatus, ExpertType, Task, TaskType,
};
use crate::orchestrator::Version;
use crate::router::{HeuristicRouter, Router};

struct TestExpert {
    meta: ExpertMetadata,
}

impl TestExpert {
    fn new(id: &str, capabilities: Vec<ExpertCapability>) -> Self {
        Self {
            meta: ExpertMetadata {
                id: ExpertId::new(),
                name: id.to_string(),
                version: Version::new(1, 0, 0),
                capabilities,
                status: ExpertStatus::Active,
                expert_type: ExpertType::Deterministic,
            },
        }
    }
}

impl Expert for TestExpert {
    fn id(&self) -> &ExpertId {
        &self.meta.id
    }

    fn metadata(&self) -> &ExpertMetadata {
        &self.meta
    }

    fn can_handle(&self, _task: &Task) -> bool {
        self.meta
            .capabilities
            .iter()
            .any(|c| matches!(c, ExpertCapability::CodeGeneration))
    }

    fn execute(
        &self,
        _task: &Task,
        _context: &ExecutionContext,
    ) -> Result<ExpertOutput, ExpertError> {
        Ok(ExpertOutput {
            expert_id: self.meta.id.clone(),
            content: "test output".to_string(),
            confidence: 0.95,
            metadata: HashMap::new(),
            trace: Vec::new(),
        })
    }
}

#[test]
fn routes_to_correct_expert_by_capability() {
    let mut registry = ExpertRegistry::new();
    let register_result = registry.register(Box::new(TestExpert::new(
        "codegen",
        vec![ExpertCapability::CodeGeneration],
    )));
    assert!(register_result.is_ok());

    let router = HeuristicRouter::new(3);
    let task = Task::new(TaskType::CodeGeneration, "write code");
    let decision = router.route(&task, &registry);
    assert!(decision.is_ok());
    let decision = decision.expect("route must succeed");
    assert!(decision.selected_experts.contains(&ExpertId::new()));
}

#[test]
fn no_matching_expert_returns_error() {
    let registry = ExpertRegistry::new();
    let router = HeuristicRouter::new(3);
    let task = Task::new(TaskType::CodeGeneration, "write code");
    let result = router.route(&task, &registry);
    assert!(result.is_err());
}

#[test]
fn respects_max_experts_limit() {
    let mut registry = ExpertRegistry::new();
    for i in 0..5 {
        let register_result = registry.register(Box::new(TestExpert::new(
            &format!("e{i}"),
            vec![ExpertCapability::CodeGeneration],
        )));
        assert!(register_result.is_ok());
    }
    let router = HeuristicRouter::new(2);
    let task = Task::new(TaskType::CodeGeneration, "write code");
    let decision = router.route(&task, &registry);
    assert!(decision.is_ok());
    let decision = decision.expect("route must succeed");
    assert!(decision.selected_experts.len() <= 2);
}
