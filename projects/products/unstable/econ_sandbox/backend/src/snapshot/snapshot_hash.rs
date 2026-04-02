use sha2::{Digest, Sha256};

pub struct SnapshotHash;

impl SnapshotHash {
    pub fn compute(snapshot: &crate::snapshot::state_snapshot::StateSnapshot) -> String {
        let canonical = format!(
            "tick={},agents={},prices={}",
            snapshot.tick.value(),
            snapshot.agent_summary,
            snapshot.price_summary,
        );
        let mut hasher = Sha256::new();
        hasher.update(canonical.as_bytes());
        hex::encode(hasher.finalize())
    }
}
