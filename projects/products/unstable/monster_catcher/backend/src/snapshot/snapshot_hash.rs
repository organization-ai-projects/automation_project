use crate::diagnostics::error::BackendError;
use crate::io::canonical_json::to_canonical_string;
use crate::snapshot::state_snapshot::StateSnapshot;
use sha2::{Digest, Sha256};

pub struct SnapshotHash;

impl SnapshotHash {
    pub fn compute(snapshot: &StateSnapshot) -> Result<String, BackendError> {
        let canonical = to_canonical_string(snapshot).map_err(BackendError::Codec)?;
        let digest = Sha256::digest(canonical.as_bytes());
        Ok(hex::encode(digest))
    }
}
