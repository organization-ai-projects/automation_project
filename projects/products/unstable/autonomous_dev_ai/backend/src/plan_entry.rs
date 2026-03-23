use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanEntry {
    pub iteration: usize,
    pub description: String,
    pub steps: Vec<String>,
}
