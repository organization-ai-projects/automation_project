#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// Global simulation configuration knobs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimConfig {
    /// How many ticks between maintenance checks per ride.
    pub maintenance_check_interval: u64,
    /// How many ticks a visitor waits in a shop before going idle again.
    pub shop_visit_ticks: u64,
    /// Enable snapshot hash checkpoints in the report.
    pub snapshot_checkpoints: bool,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            maintenance_check_interval: 20,
            shop_visit_ticks: 3,
            snapshot_checkpoints: true,
        }
    }
}
