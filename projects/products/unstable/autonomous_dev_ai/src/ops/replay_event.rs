// projects/products/unstable/autonomous_dev_ai/src/ops/replay_event.rs
use serde::{Deserialize, Serialize};

/// A single event in a run's causal timeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayEvent {
    pub sequence: usize,
    pub kind: String,
    pub payload: String,
    pub timestamp_secs: u64,
}
