// projects/products/unstable/platform_versioning/backend/src/checkout/materialized.rs
use serde::{Deserialize, Serialize};

use crate::ids::CommitId;

/// The result of a successful checkout operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Materialized {
    /// The commit that was checked out.
    pub commit_id: CommitId,
    /// Number of files written.
    pub files_written: usize,
    /// Number of files deleted (only non-zero when policy has `delete_untracked`).
    pub files_deleted: usize,
}
