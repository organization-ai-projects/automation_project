use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EngineConfig {
    pub enable_neural_assistance: bool,
    pub enable_recommendation_output: bool,
    pub max_scenarios: usize,
    pub confidence_threshold: f64,
    pub replay_deterministic: bool,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            enable_neural_assistance: false,
            enable_recommendation_output: cfg!(feature = "recommendation_output"),
            max_scenarios: 10,
            confidence_threshold: 0.6,
            replay_deterministic: true,
        }
    }
}
