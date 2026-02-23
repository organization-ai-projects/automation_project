// projects/products/unstable/platform_versioning/backend/src/merge/conflict.rs
use serde::{Deserialize, Serialize};

use crate::ids::BlobId;
use crate::indexes::SafePath;
use crate::merges::ConflictKind;

/// A single merge conflict for a specific file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Conflict {
    /// The path of the conflicting file.
    pub path: SafePath,
    /// The kind of conflict.
    pub kind: ConflictKind,
    /// The blob id of the base version (common ancestor), if present.
    pub base_blob: Option<BlobId>,
    /// The blob id of the "ours" version.
    pub ours_blob: Option<BlobId>,
    /// The blob id of the "theirs" version.
    pub theirs_blob: Option<BlobId>,
}
