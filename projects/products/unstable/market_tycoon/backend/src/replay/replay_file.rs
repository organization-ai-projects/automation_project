use serde::{Deserialize, Serialize};

use crate::events::sim_event::SimEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayFile {
    pub seed: u64,
    pub ticks: u64,
    pub events: Vec<SimEvent>,
}

impl ReplayFile {
    pub fn new(seed: u64, ticks: u64, events: Vec<SimEvent>) -> Self {
        Self {
            seed,
            ticks,
            events,
        }
    }
}
