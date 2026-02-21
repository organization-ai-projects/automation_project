use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralProposal {
    pub action: String,
    pub confidence: f64,
    pub reasoning: String,
}
