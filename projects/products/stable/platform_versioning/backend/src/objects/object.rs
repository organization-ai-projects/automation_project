// projects/products/stable/platform_versioning/backend/src/objects/object.rs
use serde::{Deserialize, Serialize};

use crate::objects::{Blob, Commit, ObjectKind, Tree};

/// A tagged union of all storable object types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Object {
    Blob(Blob),
    Tree(Tree),
    Commit(Commit),
}

impl Object {
    /// Returns the [`ObjectKind`] of this object.
    pub fn kind(&self) -> ObjectKind {
        match self {
            Self::Blob(_) => ObjectKind::Blob,
            Self::Tree(_) => ObjectKind::Tree,
            Self::Commit(_) => ObjectKind::Commit,
        }
    }

    /// Returns `true` if the object's stored id matches its recomputed address.
    pub fn verify(&self) -> bool {
        match self {
            Self::Blob(b) => b.verify(),
            Self::Tree(t) => t.verify(),
            Self::Commit(c) => c.verify(),
        }
    }
}
