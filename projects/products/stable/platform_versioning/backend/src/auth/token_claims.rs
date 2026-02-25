// projects/products/stable/platform_versioning/backend/src/auth/token_claims.rs
use serde::{Deserialize, Serialize};

use crate::auth::{PathGrant, Permission, PermissionGrant};
use crate::ids::RepoId;
use crate::indexes::SafePath;

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
    /// Optional path-scoped grants restricting access to specific repo paths.
    ///
    /// When present for a repository, only the listed paths are accessible.
    /// Tokens without path grants have unrestricted path access.
    #[serde(default)]
    pub path_grants: Vec<PathGrant>,
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
        self.expires_at.is_none_or(|exp| now_secs < exp)
    }

    /// Returns `true` if the given `path` is accessible in `repo_id` for this token.
    ///
    /// When no path grants exist for the repository the path is always accessible.
    /// When path grants exist, the path must be covered by at least one grant.
    pub fn path_is_accessible(&self, repo_id: &RepoId, path: &SafePath) -> bool {
        let repo_grants: Vec<_> = self
            .path_grants
            .iter()
            .filter(|g| &g.repo_id == repo_id)
            .collect();

        if repo_grants.is_empty() {
            return true;
        }

        repo_grants.iter().any(|g| g.allows(path))
    }
}
