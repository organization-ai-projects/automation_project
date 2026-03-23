//! projects/products/unstable/autonomous_dev_ai/src/memory.rs
use crate::value_types::ActionName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureEntry {
    pub iteration: usize,
    pub description: String,
    pub error: String,
    pub recovery_action: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectiveEvaluationEntry {
    pub iteration: usize,
    pub scores: Vec<(String, f64)>,
    pub passed: bool,
}
