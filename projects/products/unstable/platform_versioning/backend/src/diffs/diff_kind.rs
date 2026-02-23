// projects/products/unstable/platform_versioning/backend/src/diff/diff_kind.rs
use serde::{Deserialize, Serialize};

/// The kind of change represented by a [`super::DiffEntry`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiffKind {
    /// The file was added in the new revision (present in `to`, absent in `from`).
    Added,
    /// The file was deleted in the new revision (present in `from`, absent in `to`).
    Deleted,
    /// The file was modified (both present, different content).
    Modified,
    /// The file is identical in both revisions (only included when explicitly requested).
    Unchanged,
}
