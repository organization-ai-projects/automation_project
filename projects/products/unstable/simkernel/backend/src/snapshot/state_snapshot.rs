#![allow(dead_code)]
use crate::ecs::world::World;
use crate::snapshot::snapshot_hash::SnapshotHash;
use crate::time::logical_clock::LogicalClock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    pub tick: u64,
    pub turn: u64,
    pub entity_count: usize,
    pub hash: String,
}

impl StateSnapshot {
    pub fn from_world(world: &World, clock: &LogicalClock) -> Self {
        let entity_count = world.entity_count();
        let hash = SnapshotHash::compute_raw(clock.tick.0, clock.turn.0, entity_count as u64);
        Self {
            tick: clock.tick.0,
            turn: clock.turn.0,
            entity_count,
            hash,
        }
    }
}
