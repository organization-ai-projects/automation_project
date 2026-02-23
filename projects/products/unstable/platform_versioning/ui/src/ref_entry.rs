// projects/products/unstable/platform_versioning/ui/src/ref_entry.rs
use serde::{Deserialize, Serialize};

/// A ref entry for display in the repo detail view.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefEntry {
    /// The fully-qualified ref name (e.g. `heads/main`).
    pub name: String,
    /// The commit id this ref points to.
    pub commit_id: String,
}
