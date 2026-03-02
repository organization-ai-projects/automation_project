use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimConfig {
    pub ticks: u64,
    pub seed: u64,
    pub scenario_path: Option<String>,
    pub output_path: Option<String>,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            ticks: 100,
            seed: 42,
            scenario_path: None,
            output_path: None,
        }
    }
}
