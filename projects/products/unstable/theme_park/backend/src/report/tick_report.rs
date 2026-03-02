#![allow(dead_code)]
use crate::time::tick::Tick;
use serde::{Deserialize, Serialize};

/// Summary of simulation state at a specific tick checkpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickReport {
    pub tick: u64,
    pub active_visitors: usize,
    pub total_revenue: u64,
    pub reputation: i32,
    pub snapshot_hash: String,
}

impl TickReport {
    pub fn new(
        tick: Tick,
        active_visitors: usize,
        total_revenue: u64,
        reputation: i32,
        snapshot_hash: String,
    ) -> Self {
        Self {
            tick: tick.value(),
            active_visitors,
            total_revenue,
            reputation,
            snapshot_hash,
        }
    }
}
