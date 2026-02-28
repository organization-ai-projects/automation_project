use crate::snapshot::StateSnapshot;
use sha2::{Digest, Sha256};

pub struct SnapshotHash(pub String);

impl SnapshotHash {
    pub fn compute(snapshot: &StateSnapshot) -> SnapshotHash {
        let mut hasher = Sha256::new();
        hasher.update(snapshot.state_json.as_bytes());
        let result = hasher.finalize();
        SnapshotHash(hex::encode(result))
    }

    pub fn hex(&self) -> &str {
        &self.0
    }
}
