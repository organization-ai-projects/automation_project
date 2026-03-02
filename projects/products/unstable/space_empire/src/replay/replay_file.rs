use crate::events::SimEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayFile {
    pub seed: u64,
    pub scenario_hash: String,
    pub ticks_run: u64,
    pub events: Vec<SimEvent>,
}
