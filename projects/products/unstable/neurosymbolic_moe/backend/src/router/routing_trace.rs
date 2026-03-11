//! projects/products/unstable/neurosymbolic_moe/backend/src/router/routing_trace.rs
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::moe_core::{ExpertId, TaskId};

use super::routing_decision::{RoutingDecision, RoutingStrategy};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingTrace {
    pub task_id: TaskId,
    pub candidates_evaluated: usize,
    pub selected: Vec<ExpertId>,
    pub strategy: RoutingStrategy,
    pub scores: HashMap<ExpertId, f64>,
    pub reason: String,
    pub timestamp: u64,
}

impl RoutingTrace {
    pub fn from_decision(decision: &RoutingDecision, candidates_evaluated: usize) -> Self {
        Self {
            task_id: decision.task_id.clone(),
            candidates_evaluated,
            selected: decision.selected_experts.clone(),
            strategy: decision.strategy.clone(),
            scores: decision.scores.clone(),
            reason: decision.explanation.clone(),
            // Deterministic timestamp surrogate for unstable mode scaffolding.
            timestamp: candidates_evaluated as u64,
        }
    }
}
