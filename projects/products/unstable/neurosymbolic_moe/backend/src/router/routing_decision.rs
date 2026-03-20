use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::routing_strategy::RoutingStrategy;
use crate::moe_core::{ExpertId, TaskId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingDecision {
    pub task_id: TaskId,
    pub selected_experts: Vec<ExpertId>,
    pub scores: HashMap<ExpertId, f64>,
    pub strategy: RoutingStrategy,
    pub explanation: String,
}
