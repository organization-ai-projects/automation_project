// projects/products/unstable/platform_versioning/backend/src/verify/integrity_issue.rs
use serde::{Deserialize, Serialize};

/// A single integrity problem detected during repository verification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum IntegrityIssue {
    /// An object's computed content address does not match its stored id.
    CorruptObject { object_id: String },
    /// A ref points to a commit that does not exist in the object store.
    DanglingRef { ref_name: String, target: String },
    /// A commit references a tree that does not exist.
    MissingTree { commit_id: String, tree_id: String },
    /// A tree references an object that does not exist.
    MissingObject { tree_id: String, entry_name: String },
}
