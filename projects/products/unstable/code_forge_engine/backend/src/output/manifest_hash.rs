// projects/products/unstable/code_forge_engine/backend/src/output/manifest_hash.rs
use sha2::{Digest, Sha256};

pub struct ManifestHash;

impl ManifestHash {
    pub fn compute(canonical_json: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(canonical_json.as_bytes());
        hex::encode(hasher.finalize())
    }
}
