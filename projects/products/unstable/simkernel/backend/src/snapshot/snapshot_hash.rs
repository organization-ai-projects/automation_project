#![allow(dead_code)]
use crate::snapshot::state_snapshot::StateSnapshot;
use sha2::{Digest, Sha256};

pub struct SnapshotHash;

impl SnapshotHash {
    pub fn compute(snapshot: &StateSnapshot) -> String {
        Self::compute_raw(snapshot.tick, snapshot.turn, snapshot.entity_count as u64)
    }

    pub fn compute_raw(tick: u64, turn: u64, entity_count: u64) -> String {
        let mut hasher = Sha256::new();
        hasher.update(tick.to_le_bytes());
        hasher.update(turn.to_le_bytes());
        hasher.update(entity_count.to_le_bytes());
        hex::encode(hasher.finalize())
    }
}
