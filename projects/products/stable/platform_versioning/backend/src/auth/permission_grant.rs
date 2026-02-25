// projects/products/stable/platform_versioning/backend/src/auth/permission_grant.rs
use serde::{Deserialize, Serialize};

use crate::auth::Permission;
use crate::ids::RepoId;

/// A single permission grant within [`super::TokenClaims`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionGrant {
    /// The repository this grant applies to, or `None` for all repositories.
    pub repo_id: Option<RepoId>,
    /// The granted permission.
    pub permission: Permission,
}
