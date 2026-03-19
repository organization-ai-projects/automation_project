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
