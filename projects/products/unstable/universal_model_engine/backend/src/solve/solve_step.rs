use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolveStep {
    pub index: u64,
    pub transition_id: String,
}
