use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimReport {
    pub run_hash: String,
    pub seed: u64,
    pub ticks: u64,
    pub event_count: usize,
    pub snapshot_hash: String,
}
