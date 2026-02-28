#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunReport {
    pub pack_kind: String,
    pub seed: u64,
    pub ticks: u64,
    pub run_hash: String,
    pub event_count: usize,
}

impl RunReport {
    pub fn new(
        pack_kind: String,
        seed: u64,
        ticks: u64,
        run_hash: String,
        event_count: usize,
    ) -> Self {
        Self {
            pack_kind,
            seed,
            ticks,
            run_hash,
            event_count,
        }
    }
}
