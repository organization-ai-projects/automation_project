// projects/products/unstable/platform_versioning/backend/src/sync/ref_update.rs
use serde::{Deserialize, Serialize};

use crate::ids::CommitId;
use crate::refs_store::RefName;
use crate::sync::RefUpdatePolicy;

/// A request to update a single ref to a new commit id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefUpdate {
    /// The ref to update.
    pub ref_name: RefName,
    /// The new target commit id.
    pub new_commit: CommitId,
    /// The policy governing this update.
    pub policy: RefUpdatePolicy,
}
