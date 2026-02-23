// projects/products/unstable/platform_versioning/backend/src/diff/diff_entry.rs
use serde::{Deserialize, Serialize};

use crate::diff::{ContentClass, DiffKind};
use crate::ids::BlobId;
use crate::index::SafePath;

/// A single file change in a [`super::Diff`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffEntry {
    /// The path of the file (relative to the repository root).
    pub path: SafePath,
    /// The kind of change.
    pub kind: DiffKind,
    /// Content class of the base version (or `None` for added files).
    pub from_class: Option<ContentClass>,
    /// The blob id of the base version (or `None` for added files).
    pub from_blob: Option<BlobId>,
    /// Content class of the new version (or `None` for deleted files).
    pub to_class: Option<ContentClass>,
    /// The blob id of the new version (or `None` for deleted files).
    pub to_blob: Option<BlobId>,
}
