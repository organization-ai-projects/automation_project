use sha2::{Digest, Sha256};

use crate::diagnostics::error::Error;
use crate::index::inverted_index::InvertedIndex;

/// A persisted snapshot of the inverted index with a checksum.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct IndexSnapshot {
    pub(crate) index_json: String,
    pub(crate) checksum: String,
}

impl IndexSnapshot {
    pub(crate) fn from_index(index: &InvertedIndex) -> Result<Self, Error> {
        let json = crate::persistence::snapshot_codec::SnapshotCodec::canonical_index_json(index)?;
        let checksum = Self::compute_checksum(&json);
        Ok(Self {
            index_json: json,
            checksum,
        })
    }

    pub(crate) fn to_index(&self) -> Result<InvertedIndex, Error> {
        let expected = Self::compute_checksum(&self.index_json);
        if self.checksum != expected {
            return Err(Error::ChecksumMismatch {
                expected,
                actual: self.checksum.clone(),
            });
        }
        common_json::from_str(&self.index_json)
            .map_err(|e| Error::Deserialization(e.to_string()))
    }

    fn compute_checksum(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hex::encode(hasher.finalize())
    }
}
