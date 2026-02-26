// projects/products/stable/platform_versioning/backend/src/objects/blob.rs
use serde::{Deserialize, Serialize};

use crate::ids::BlobId;
use crate::objects::HashDigest;

/// An immutable content-addressed byte sequence.
///
/// # Encoding format (version 1)
/// ```text
/// b"blob\x00" ++ little-endian u64 length ++ raw content bytes
/// ```
/// The SHA-256 of the above byte sequence is the canonical [`BlobId`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Blob {
    /// The content address of this blob.
    pub id: BlobId,
    /// The raw byte content.
    pub content: Vec<u8>,
}

/// Current encoding format version for blobs.
const BLOB_FORMAT_VERSION: u8 = 1;

impl Blob {
    /// Creates a `Blob` from raw bytes, computing the content address.
    pub fn from_bytes(content: Vec<u8>) -> Self {
        let id = Self::compute_id(&content);
        Self { id, content }
    }

    /// Computes the [`BlobId`] for the given raw bytes without allocating a full `Blob`.
    pub fn compute_id(content: &[u8]) -> BlobId {
        let prefix = b"blob\x00";
        let len_bytes = (content.len() as u64).to_le_bytes();
        let digest =
            HashDigest::compute_parts(&[&[BLOB_FORMAT_VERSION], prefix, &len_bytes, content]);
        BlobId::from_bytes(&digest)
    }

    /// Validates that the stored `id` matches the recomputed address of `content`.
    pub fn verify(&self) -> bool {
        Self::compute_id(&self.content) == self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_is_deterministic() {
        let a = Blob::from_bytes(b"hello world".to_vec());
        let b = Blob::from_bytes(b"hello world".to_vec());
        assert_eq!(a.id, b.id);
    }

    #[test]
    fn different_content_different_id() {
        let a = Blob::from_bytes(b"hello".to_vec());
        let b = Blob::from_bytes(b"world".to_vec());
        assert_ne!(a.id, b.id);
    }

    #[test]
    fn verify_intact() {
        let blob = Blob::from_bytes(b"test data".to_vec());
        assert!(blob.verify());
    }

    #[test]
    fn verify_corrupt() {
        let mut blob = Blob::from_bytes(b"test data".to_vec());
        blob.content[0] ^= 0xff;
        assert!(!blob.verify());
    }
}
