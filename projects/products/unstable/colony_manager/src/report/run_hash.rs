use sha2::{Digest, Sha256};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RunHash(pub String);

impl RunHash {
    pub fn compute(canonical_bytes: &[u8]) -> Self {
        let hash = Sha256::digest(canonical_bytes);
        Self(hex::encode(hash))
    }
}
