// projects/products/unstable/platform_versioning/backend/src/checkout/checkout_policy.rs
use serde::{Deserialize, Serialize};

/// Policy that governs how checkout handles existing files.
///
/// # Overwrite policy
/// - `Overwrite`: existing files at the destination are replaced by the revision
///   contents (default for most operations).
/// - `FailOnConflict`: the checkout is aborted if any destination file already
///   exists with different content.
///
/// # Delete policy
/// Files present in the working tree but absent from the target revision are
/// deleted when `delete_untracked` is `true`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckoutPolicy {
    /// Whether to overwrite files that exist at the destination.
    pub overwrite: bool,
    /// Whether to delete files that exist in the destination but not in the
    /// target revision.
    pub delete_untracked: bool,
}

impl CheckoutPolicy {
    /// The default policy: overwrite changed files, do not delete untracked.
    pub fn overwrite() -> Self {
        Self {
            overwrite: true,
            delete_untracked: false,
        }
    }

    /// Clean policy: overwrite and delete untracked.
    pub fn clean() -> Self {
        Self {
            overwrite: true,
            delete_untracked: true,
        }
    }

    /// Safe policy: fail on conflict, do not delete untracked.
    pub fn safe() -> Self {
        Self {
            overwrite: false,
            delete_untracked: false,
        }
    }
}
