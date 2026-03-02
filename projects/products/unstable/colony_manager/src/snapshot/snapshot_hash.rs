use crate::snapshot::state_snapshot::StateSnapshot;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct SnapshotHash(pub String);

impl SnapshotHash {
    pub fn compute(snapshot: &StateSnapshot) -> Self {
        let json = serde_json::to_vec(snapshot).expect("snapshot serializable");
        let hash = Sha256::digest(&json);
        Self(hex::encode(hash))
    }
}
