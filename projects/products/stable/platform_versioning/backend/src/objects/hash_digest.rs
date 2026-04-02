// projects/products/stable/platform_versioning/backend/src/objects/hash_digest.rs
use sha2::{Digest, Sha256};

/// Computes the SHA-256 digest of `data`.
///
/// The encoding prefix (`"blob "`, `"tree "`, etc.) must be prepended by the
/// caller before invoking this function so that different object kinds with
/// identical raw payloads produce different addresses.
pub struct HashDigest;

impl HashDigest {
    /// Returns the 32-byte SHA-256 of `data`.
    pub fn compute(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    /// Returns the 32-byte SHA-256 of the concatenation of all `parts`.
    pub fn compute_parts(parts: &[&[u8]]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        for part in parts {
            hasher.update(part);
        }
        hasher.finalize().into()
    }
}
