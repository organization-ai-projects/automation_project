// projects/products/unstable/autonomous_dev_ai/src/symbolic/plan.rs
use serde::{Deserialize, Serialize};

use crate::symbolic::PlanStep;
// Execution plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub goal: String,
    pub steps: Vec<PlanStep>,
    pub risk_level: String,
}

impl Plan {
    pub fn new(goal: String) -> Self {
        Self {
            goal,
            steps: Vec::new(),
            risk_level: "low".to_string(),
        }
    }

    pub fn add_step(&mut self, step: PlanStep) {
        self.steps.push(step);
    }
}
