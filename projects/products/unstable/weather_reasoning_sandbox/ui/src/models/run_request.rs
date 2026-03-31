use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunRequest {
    pub seed: u64,
    pub ticks: u64,
    pub dataset: Option<String>,
}

impl Default for RunRequest {
    fn default() -> Self {
        Self {
            seed: 42,
            ticks: 10,
            dataset: None,
        }
    }
}
