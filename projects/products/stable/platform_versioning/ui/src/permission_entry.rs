// projects/products/stable/platform_versioning/ui/src/permission_entry.rs
use serde::{Deserialize, Serialize};

/// A single permission entry displayed in the admin permission panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionEntry {
    /// The subject (user identifier) this entry applies to.
    pub subject: String,
    /// The repository scope, or `"global"` for cross-repo permissions.
    pub repo_id: String,
    /// The permission level: `"read"`, `"write"`, or `"admin"`.
    pub permission: String,
}
