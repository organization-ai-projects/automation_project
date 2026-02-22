// projects/products/unstable/autonomous_dev_ai/src/agent_config.rs
use serde::{Deserialize, Serialize};

use crate::{
    neural_config::NeuralConfig,
    objectives::{Objective, default_objectives},
    symbolic_config::SymbolicConfig,
};
// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
#[serde(default)]
pub struct AgentConfig {
    pub agent_name: String,
    pub execution_mode: String,
    pub neural: NeuralConfig,
    pub symbolic: SymbolicConfig,
    pub objectives: Vec<Objective>,
    pub max_iterations: usize,
    pub timeout_seconds: Option<u64>,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            agent_name: "autonomous_dev_ai".to_string(),
            execution_mode: "ci_bound".to_string(),
            neural: NeuralConfig::default(),
            symbolic: SymbolicConfig::default(),
            objectives: default_objectives(),
            max_iterations: 100,
            timeout_seconds: Some(3600),
        }
    }
}
