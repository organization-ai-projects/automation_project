// projects/products/unstable/platform_versioning/backend/src/refs_store/ref_target.rs
use serde::{Deserialize, Serialize};

use crate::ids::CommitId;

/// The target that a ref points to.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RefTarget {
    /// Points directly to a commit.
    Commit(CommitId),
}

impl RefTarget {
    /// Returns the [`CommitId`] this ref resolves to.
    pub fn commit_id(&self) -> &CommitId {
        let Self::Commit(id) = self;
        id
    }
}
