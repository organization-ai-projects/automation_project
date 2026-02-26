// projects/products/stable/platform_versioning/backend/src/merge/conflict_kind.rs
use serde::{Deserialize, Serialize};

/// The kind of merge conflict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictKind {
    /// Both sides modified the same file with incompatible content.
    Content,
    /// One side deleted the file while the other modified it.
    DeleteModify,
    /// Both sides added a file at the same path with different content.
    AddAdd,
    /// A binary file has conflicting versions (cannot be auto-merged).
    Binary,
}
