use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimConfig {
    pub max_ticks: u64,
    pub seed: u64,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            max_ticks: 100,
            seed: 42,
        }
    }
}
