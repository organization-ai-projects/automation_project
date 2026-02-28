// projects/products/unstable/autonomy_orchestrator_ai/src/domain/planner_edge.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannerEdge {
    pub from: String,
    pub to: String,
    pub condition_code: String,
}
