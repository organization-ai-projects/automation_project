#![allow(dead_code)]
use sha2::{Digest, Sha256};

/// Computes a deterministic hash of the simulation state at a tick.
pub struct SnapshotHash;

impl SnapshotHash {
    pub fn compute(tick: u64, active_visitors: u64, total_revenue: u64, reputation: i32) -> String {
        let mut hasher = Sha256::new();
        hasher.update(tick.to_le_bytes());
        hasher.update(active_visitors.to_le_bytes());
        hasher.update(total_revenue.to_le_bytes());
        hasher.update(reputation.to_le_bytes());
        hex::encode(hasher.finalize())
    }
}
