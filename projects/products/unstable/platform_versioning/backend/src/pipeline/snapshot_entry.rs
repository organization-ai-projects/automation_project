// projects/products/unstable/platform_versioning/backend/src/pipeline/snapshot_entry.rs
use serde::{Deserialize, Serialize};

use crate::ids::BlobId;
use crate::indexes::SafePath;

/// A single file entry in a [`super::Snapshot`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotEntry {
    /// The safe path of this file.
    pub path: SafePath,
    /// The content address of this file's content.
    pub blob_id: BlobId,
}
