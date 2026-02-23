// projects/products/unstable/platform_versioning/backend/src/objects/tree_entry_kind.rs
use serde::{Deserialize, Serialize};

/// The kind of an entry in a tree object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreeEntryKind {
    /// A regular file (blob).
    Blob,
    /// A nested directory (subtree).
    Tree,
}
