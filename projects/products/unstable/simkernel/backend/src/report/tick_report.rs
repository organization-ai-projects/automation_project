#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickReport {
    pub tick: u64,
    pub event_count: usize,
    pub snapshot_hash: String,
}
