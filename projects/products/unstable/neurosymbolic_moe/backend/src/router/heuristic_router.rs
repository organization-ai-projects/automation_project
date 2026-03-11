use std::collections::HashMap;

use crate::expert_registry::ExpertRegistry;
use crate::moe_core::{ExpertCapability, ExpertId, MoeError, Task, TaskType};

use super::router_trait::Router;
use super::routing_decision::{RoutingDecision, RoutingStrategy};

pub struct HeuristicRouter {
    pub max_experts: usize,
}

impl HeuristicRouter {
    pub fn new(max_experts: usize) -> Self {
        Self { max_experts }
    }

    fn preferred_capability(task_type: &TaskType) -> ExpertCapability {
        match task_type {
            TaskType::CodeGeneration => ExpertCapability::CodeGeneration,
            TaskType::CodeAnalysis => ExpertCapability::Validation,
            TaskType::CodeTransformation => ExpertCapability::CodeTransformation,
            TaskType::Refactoring => ExpertCapability::CodeTransformation,
            TaskType::Documentation => ExpertCapability::Summarization,
            TaskType::Planning => ExpertCapability::IssuePlanning,
            TaskType::Retrieval => ExpertCapability::Retrieval,
            TaskType::Evaluation => ExpertCapability::Evaluation,
            TaskType::Validation => ExpertCapability::Validation,
            TaskType::Custom(name) => ExpertCapability::Custom(name.clone()),
        }
    }
}

impl Default for HeuristicRouter {
    fn default() -> Self {
        Self { max_experts: 3 }
    }
}

impl Router for HeuristicRouter {
    fn route(&self, task: &Task, registry: &ExpertRegistry) -> Result<RoutingDecision, MoeError> {
        let capability = Self::preferred_capability(&task.task_type);
        let mut scores: HashMap<ExpertId, f64> = HashMap::new();

        // Score experts that match the preferred capability
        let capability_matches = registry.find_by_capability(&capability);
        for expert in &capability_matches {
            scores.insert(expert.id().clone(), 1.0);
        }

        // Fall back to can_handle if no capability match
        if scores.is_empty() {
            let task_matches = registry.find_for_task(task);
            for expert in &task_matches {
                scores.insert(expert.id().clone(), 0.7);
            }
        }

        if scores.is_empty() {
            return Err(MoeError::NoExpertFound(format!(
                "no expert found for task '{}' of type {:?}",
                task.id.as_str(),
                task.task_type
            )));
        }

        // Sort by score descending and take top N
        let mut ranked: Vec<(ExpertId, f64)> =
            scores.iter().map(|(k, v)| (k.clone(), *v)).collect();
        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        ranked.truncate(self.max_experts);

        let selected_experts: Vec<ExpertId> = ranked.iter().map(|(id, _)| id.clone()).collect();

        let strategy = if selected_experts.len() == 1 {
            RoutingStrategy::SingleExpert
        } else {
            RoutingStrategy::MultiExpert
        };

        let explanation = format!(
            "Routed task '{}' ({:?}) to {} expert(s) via {:?} strategy using {} matching",
            task.id.as_str(),
            task.task_type,
            selected_experts.len(),
            strategy,
            if ranked.first().is_some_and(|(_, s)| *s >= 1.0) {
                "capability"
            } else {
                "can_handle"
            },
        );

        Ok(RoutingDecision {
            task_id: task.id.clone(),
            selected_experts,
            scores,
            strategy,
            explanation,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::moe_core::{
        ExecutionContext, Expert, ExpertError, ExpertId, ExpertMetadata, ExpertOutput,
        ExpertStatus, ExpertType, TaskType,
    };

    struct TestExpert {
        meta: ExpertMetadata,
    }

    impl TestExpert {
        fn new(id: &str, capabilities: Vec<ExpertCapability>) -> Self {
            Self {
                meta: ExpertMetadata {
                    id: ExpertId::new(id),
                    name: id.to_string(),
                    version: "1.0.0".to_string(),
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

    fn make_registry_with_expert(
        id: &str,
        caps: Vec<ExpertCapability>,
    ) -> (ExpertRegistry, ExpertId) {
        let mut reg = ExpertRegistry::new();
        reg.register(Box::new(TestExpert::new(id, caps))).unwrap();
        (reg, ExpertId::new(id))
    }

    #[test]
    fn routes_to_correct_expert_by_capability() {
        let (registry, _) =
            make_registry_with_expert("codegen", vec![ExpertCapability::CodeGeneration]);
        let router = HeuristicRouter::new(3);
        let task = Task::new("t1", TaskType::CodeGeneration, "write code");
        let decision = router.route(&task, &registry).unwrap();
        assert!(
            decision
                .selected_experts
                .contains(&ExpertId::new("codegen"))
        );
    }

    #[test]
    fn no_matching_expert_returns_error() {
        let registry = ExpertRegistry::new();
        let router = HeuristicRouter::new(3);
        let task = Task::new("t1", TaskType::CodeGeneration, "write code");
        let result = router.route(&task, &registry);
        assert!(result.is_err());
    }

    #[test]
    fn respects_max_experts_limit() {
        let mut registry = ExpertRegistry::new();
        for i in 0..5 {
            registry
                .register(Box::new(TestExpert::new(
                    &format!("e{i}"),
                    vec![ExpertCapability::CodeGeneration],
                )))
                .unwrap();
        }
        let router = HeuristicRouter::new(2);
        let task = Task::new("t1", TaskType::CodeGeneration, "write code");
        let decision = router.route(&task, &registry).unwrap();
        assert!(decision.selected_experts.len() <= 2);
    }
}
