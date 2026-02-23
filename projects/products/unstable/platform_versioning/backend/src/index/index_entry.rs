// projects/products/unstable/platform_versioning/backend/src/index/index_entry.rs
use serde::{Deserialize, Serialize};

use crate::ids::BlobId;
use crate::index::SafePath;

/// A single entry in the staging index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexEntry {
    /// The safe relative path within the working tree.
    pub path: SafePath,
    /// The content address of the staged blob.
    pub blob_id: BlobId,
}
