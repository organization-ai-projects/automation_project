use sha2::{Digest, Sha256};

pub struct RunHash;

impl RunHash {
    pub fn compute(seed: u64, ticks: u64, event_count: usize, net_profit: i64) -> String {
        let canonical = format!(
            "seed={},ticks={},events={},profit={}",
            seed, ticks, event_count, net_profit
        );
        let mut hasher = Sha256::new();
        hasher.update(canonical.as_bytes());
        hex::encode(hasher.finalize())
    }
}
