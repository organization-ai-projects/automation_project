use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Budget {
    pub max_actions_per_plan: usize,
    pub max_planner_iterations: usize,
    pub max_time_budget_ms: u64,
}

impl Default for Budget {
    fn default() -> Self {
        Self {
            max_actions_per_plan: 100,
            max_planner_iterations: 10,
            max_time_budget_ms: 5000,
        }
    }
}
