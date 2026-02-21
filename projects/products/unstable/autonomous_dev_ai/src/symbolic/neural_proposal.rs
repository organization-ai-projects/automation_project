// projects/products/unstable/autonomous_dev_ai/src/symbolic/neural_proposal.rs
use crate::value_types::ActionName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralProposal {
    pub action: ActionName,
    pub confidence: f64,
    pub reasoning: String,
}
