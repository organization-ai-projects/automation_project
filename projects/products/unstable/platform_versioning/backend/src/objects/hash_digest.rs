// projects/products/unstable/platform_versioning/backend/src/objects/hash_digest.rs
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_blob_is_deterministic() {
        let a = HashDigest::compute(b"");
        let b = HashDigest::compute(b"");
        assert_eq!(a, b);
    }

    #[test]
    fn different_inputs_differ() {
        let a = HashDigest::compute(b"hello");
        let b = HashDigest::compute(b"world");
        assert_ne!(a, b);
    }

    #[test]
    fn parts_matches_concat() {
        let full = HashDigest::compute(b"helloworld");
        let parts = HashDigest::compute_parts(&[b"hello", b"world"]);
        assert_eq!(full, parts);
    }
}
