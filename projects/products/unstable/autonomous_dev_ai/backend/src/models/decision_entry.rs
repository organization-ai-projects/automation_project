//! projects/products/unstable/autonomous_dev_ai/src/models/decision_entry.rs
use serde::{Deserialize, Serialize};

use crate::value_types::ActionName;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionEntry {
    pub iteration: usize,
    pub description: String,
    pub neural_suggestion: Option<String>,
    pub symbolic_decision: ActionName,
}
