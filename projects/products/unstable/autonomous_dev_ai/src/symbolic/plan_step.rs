// projects/products/unstable/autonomous_dev_ai/src/symbolic/plan_step.rs
use serde::{Deserialize, Serialize};

// Plan step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanStep {
    pub description: String,
    pub tool: String,
    pub args: Vec<String>,
    pub verification: String,
}
