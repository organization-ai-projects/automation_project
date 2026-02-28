use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::diagnostics::error::SpaceDiploWarsError;
use crate::io::json_codec::JsonCodec;

use super::state_snapshot::StateSnapshot;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotHash(pub String);

impl SnapshotHash {
    /// Compute SHA-256 of canonical JSON bytes of the snapshot.
    pub fn compute(snapshot: &StateSnapshot) -> Result<Self, SpaceDiploWarsError> {
        let json = JsonCodec::encode(snapshot)?;
        let bytes = json.as_bytes();
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let result = hasher.finalize();
        Ok(Self(hex::encode(result)))
    }
}
