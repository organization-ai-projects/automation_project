// projects/products/unstable/autonomous_dev_ai/src/memory.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct DecisionEntry {
    pub iteration: usize,
    pub description: String,
    pub neural_suggestion: Option<String>,
    pub symbolic_decision: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct FailureEntry {
    pub iteration: usize,
    pub description: String,
    pub error: String,
    pub recovery_action: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct ObjectiveEvaluationEntry {
    pub iteration: usize,
    pub scores: Vec<(String, f64)>,
    pub passed: bool,
}
