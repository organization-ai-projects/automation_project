#![allow(dead_code)]
use crate::sim::sim_state::SimState;
use crate::snapshot::snapshot_hash::SnapshotHash;
use serde::{Deserialize, Serialize};

/// Lightweight summary of simulation state at a given tick.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    pub tick: u64,
    pub visitor_count: usize,
    pub active_visitor_count: usize,
    pub ride_count: usize,
    pub shop_count: usize,
    pub total_revenue: u64,
    pub reputation_score: i32,
    pub hash: String,
}

impl StateSnapshot {
    pub fn from_state(state: &SimState) -> Self {
        let tick = state.clock.tick.value();
        let visitor_count = state.visitors.len();
        let active_visitor_count = state.active_visitor_count();
        let ride_count = state.rides.len();
        let shop_count = state.shops.len();
        let total_revenue = state.total_revenue();
        let reputation_score = state.reputation.score;
        let hash = SnapshotHash::compute(
            tick,
            active_visitor_count as u64,
            total_revenue,
            reputation_score,
        );
        Self {
            tick,
            visitor_count,
            active_visitor_count,
            ride_count,
            shop_count,
            total_revenue,
            reputation_score,
            hash,
        }
    }
}
