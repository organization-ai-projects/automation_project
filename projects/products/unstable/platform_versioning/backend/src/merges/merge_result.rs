// projects/products/unstable/platform_versioning/backend/src/merge/merge_result.rs
use serde::{Deserialize, Serialize};

use crate::ids::CommitId;
use crate::merges::Conflict;

/// The outcome of a merge attempt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum MergeResult {
    /// The merge succeeded cleanly; `commit_id` is the new merge commit.
    Clean { commit_id: CommitId },
    /// The merge produced conflicts that must be resolved manually.
    Conflicted { conflicts: Vec<Conflict> },
}
