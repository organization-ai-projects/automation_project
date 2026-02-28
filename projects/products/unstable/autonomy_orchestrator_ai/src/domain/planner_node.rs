// projects/products/unstable/autonomy_orchestrator_ai/src/domain/planner_node.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannerNode {
    pub id: String,
    pub action: String,
}
