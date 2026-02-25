// projects/products/stable/platform_versioning/backend/src/refs_store/head_state.rs
use serde::{Deserialize, Serialize};

use crate::refs_store::RefName;

/// The state of the HEAD pointer.
///
/// HEAD is a symbolic pointer: normally it tracks a branch, but after a
/// detached-head checkout it points directly to a commit id string.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeadState {
    /// HEAD tracks a branch.
    Branch(RefName),
    /// HEAD is detached and points to a specific commit hex string.
    Detached(String),
    /// No commit exists yet (initial state before the first commit).
    Unborn(RefName),
}
