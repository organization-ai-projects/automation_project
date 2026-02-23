// projects/products/unstable/platform_versioning/backend/src/merge/conflict_kind.rs
use serde::{Deserialize, Serialize};

/// The kind of merge conflict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictKind {
    /// Both sides modified the same file with incompatible content.
    ContentConflict,
    /// One side deleted the file while the other modified it.
    DeleteModifyConflict,
    /// Both sides added a file at the same path with different content.
    AddAddConflict,
    /// A binary file has conflicting versions (cannot be auto-merged).
    BinaryConflict,
}
