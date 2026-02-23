// projects/products/unstable/platform_versioning/backend/src/pipeline/commit_result.rs
use serde::{Deserialize, Serialize};

use crate::ids::CommitId;

/// The result of a successful commit operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitResult {
    /// The content address of the newly created commit.
    pub commit_id: CommitId,
    /// The branch ref that was updated, if any.
    pub updated_ref: Option<String>,
}
