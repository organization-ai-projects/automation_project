use sha2::{Digest, Sha256};

pub struct RunHash;

impl RunHash {
    pub fn compute(seed: u64, ticks: u64, event_count: usize, snapshot_hash: &str) -> String {
        let canonical = format!(
            "seed={},ticks={},events={},snapshot={}",
            seed, ticks, event_count, snapshot_hash
        );
        let mut hasher = Sha256::new();
        hasher.update(canonical.as_bytes());
        hex::encode(hasher.finalize())
    }
}
