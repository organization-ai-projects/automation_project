use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct PlanEntry {
    pub iteration: usize,
    pub description: String,
    pub steps: Vec<String>,
}
