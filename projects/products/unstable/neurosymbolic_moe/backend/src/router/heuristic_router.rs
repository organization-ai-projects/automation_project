//! projects/products/unstable/neurosymbolic_moe/backend/src/router/heuristic_router.rs
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
