// projects/products/unstable/autonomous_dev_ai/src/objectives.rs

use serde::{Deserialize, Serialize};

/// Multi-objective system for agent decision making
#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct Objective {
    pub name: String,
    pub weight: f64,
    pub hard: bool,
    pub threshold: Option<f64>,
}

impl Objective {
    pub fn new(name: String, weight: f64, hard: bool) -> Self {
        Self {
            name,
            weight,
            hard,
            threshold: None,
        }
    }

    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.threshold = Some(threshold);
        self
    }
}

/// Default objectives as specified in the requirements
pub fn default_objectives() -> Vec<Objective> {
    vec![
        Objective::new("task_completion".to_string(), 1.0, true).with_threshold(1.0),
        Objective::new("policy_safety".to_string(), 1.0, true).with_threshold(1.0),
        Objective::new("tests_pass".to_string(), 0.9, true).with_threshold(1.0),
        Objective::new("minimal_diff".to_string(), 0.6, false),
        Objective::new("time_budget".to_string(), 0.4, false),
    ]
}
