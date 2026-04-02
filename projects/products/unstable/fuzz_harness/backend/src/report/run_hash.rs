use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RunHash(pub(crate) String);

impl RunHash {
    pub(crate) fn compute(canonical_bytes: &[u8]) -> Self {
        let hash = Sha256::digest(canonical_bytes);
        Self(hex::encode(hash))
    }
}
