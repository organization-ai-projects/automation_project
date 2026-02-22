// projects/products/unstable/autonomous_dev_ai/src/neural/intent_interpretation.rs
use serde::{Deserialize, Serialize};

/// Intent interpretation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentInterpretation {
    pub goal: String,
    pub constraints: Vec<String>,
    pub confidence: f64,
}
