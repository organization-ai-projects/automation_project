use serde::{Deserialize, Serialize};

use crate::SimEngine;
use crate::events::event_log::EventLog;
use crate::snapshot::snapshot_hash::SnapshotHash;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    pub tick: u64,
    pub event_count: usize,
    pub hash: String,
}

impl StateSnapshot {
    pub fn capture(engine: &SimEngine, event_log: &EventLog) -> Self {
        let tick = engine.clock.current().value();
        let event_count = event_log.len();
        let canonical = format!(
            "tick={},events={},ledger_profit={},rng={}",
            tick,
            event_count,
            engine.ledger.net_profit(),
            engine.rng_state,
        );
        let hash = SnapshotHash::compute(&canonical);
        Self {
            tick,
            event_count,
            hash,
        }
    }
}
