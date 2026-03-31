use sha2::{Digest, Sha256};

use crate::state::StoryState;

pub struct SnapshotHash;

impl SnapshotHash {
    pub fn compute(state: &StoryState) -> String {
        let canonical = state.canonical_string();
        let mut hasher = Sha256::new();
        hasher.update(canonical.as_bytes());
        hex::encode(hasher.finalize())
    }
}
