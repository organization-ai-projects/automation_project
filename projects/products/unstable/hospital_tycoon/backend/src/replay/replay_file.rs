// projects/products/unstable/hospital_tycoon/backend/src/replay/replay_file.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayFile {
    pub seed: u64,
    pub ticks: u64,
    pub scenario_name: String,
    pub events: Vec<String>,
}

impl ReplayFile {
    pub fn new(seed: u64, ticks: u64, scenario_name: String) -> Self {
        Self {
            seed,
            ticks,
            scenario_name,
            events: Vec::new(),
        }
    }
}
