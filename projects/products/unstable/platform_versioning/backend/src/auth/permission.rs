// projects/products/unstable/platform_versioning/backend/src/auth/permission.rs
use serde::{Deserialize, Serialize};

/// Repository-scoped permissions.
///
/// Permissions are checked per-repository, per-operation. Having `Write` does
/// not imply `Read`; callers must check each required permission independently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Permission {
    /// Read repository metadata, refs, commits, trees, and blobs.
    Read,
    /// Push commits, update refs (fast-forward only by default).
    Write,
    /// Force-push, delete refs, update repo metadata.
    Admin,
}
