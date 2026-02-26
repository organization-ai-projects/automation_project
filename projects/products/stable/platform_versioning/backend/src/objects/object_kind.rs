// projects/products/stable/platform_versioning/backend/src/objects/object_kind.rs
use serde::{Deserialize, Serialize};

/// The kind of a stored object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectKind {
    /// A raw byte sequence (file content).
    Blob,
    /// A directory snapshot.
    Tree,
    /// A versioning record pointing to a tree and its parents.
    Commit,
}
