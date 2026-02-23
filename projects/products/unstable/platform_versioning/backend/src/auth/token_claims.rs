// projects/products/unstable/platform_versioning/backend/src/auth/token_claims.rs
use serde::{Deserialize, Serialize};

use crate::auth::{Permission, PermissionGrant};
use crate::ids::RepoId;

/// The decoded claims embedded in a verified [`super::AuthToken`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenClaims {
    /// The subject (user identifier).
    pub subject: String,
    /// Repository-scoped permissions granted to this token.
    /// A `None` repository id grants the permission on all repositories.
    pub grants: Vec<PermissionGrant>,
    /// Token expiry as a Unix timestamp, or `None` for non-expiring tokens.
    pub expires_at: Option<u64>,
}

impl TokenClaims {
    /// Returns `true` if this token grants `permission` on `repo`.
    pub fn has_permission(&self, repo_id: &RepoId, permission: Permission) -> bool {
        self.grants.iter().any(|g| {
            g.permission == permission
                && (g.repo_id.is_none() || g.repo_id.as_ref() == Some(repo_id))
        })
    }

    /// Returns `true` if the token is not expired at the given Unix timestamp.
    pub fn is_valid_at(&self, now_secs: u64) -> bool {
        self.expires_at.map_or(true, |exp| now_secs < exp)
    }
}
