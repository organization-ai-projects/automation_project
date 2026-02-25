// projects/products/stable/platform_versioning/backend/src/history/history_entry.rs
use serde::{Deserialize, Serialize};

use crate::ids::CommitId;

/// A single entry in a history traversal result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// The content address of this commit.
    pub commit_id: CommitId,
    /// The author identifier.
    pub author: String,
    /// The commit message.
    pub message: String,
    /// Unix timestamp when this commit was created.
    pub timestamp_secs: u64,
    /// Parent commit ids.
    pub parent_ids: Vec<CommitId>,
}
