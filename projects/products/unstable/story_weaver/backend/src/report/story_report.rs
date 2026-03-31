use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryReport {
    pub run_hash: String,
    pub seed: u64,
    pub steps_taken: u64,
    pub event_count: usize,
    pub snapshot_hash: String,
    pub title: String,
}
