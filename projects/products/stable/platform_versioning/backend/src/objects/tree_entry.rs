// projects/products/stable/platform_versioning/backend/src/objects/tree_entry.rs
use serde::{Deserialize, Serialize};

use crate::ids::ObjectId;
use crate::objects::TreeEntryKind;

/// A single entry within a [`super::Tree`].
///
/// Entries are sorted lexicographically by `name` when stored, which ensures
/// deterministic tree hashes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TreeEntry {
    /// The name of this entry within its parent directory.
    pub name: String,
    /// The kind of object this entry points to.
    pub kind: TreeEntryKind,
    /// The content address of the referenced object.
    pub id: ObjectId,
}
