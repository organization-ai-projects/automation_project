// projects/products/stable/platform_versioning/backend/src/auth/path_grant.rs
use serde::{Deserialize, Serialize};

use crate::ids::RepoId;
use crate::indexes::SafePath;

/// A path-scoped permission grant restricting a token to specific repository paths.
///
/// When a token contains path grants for a repository, only the listed paths
/// (and their children) are accessible. Tokens without any path grants for a
/// repository have unrestricted path access within that repository.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PathGrant {
    /// The repository this path restriction applies to.
    pub repo_id: RepoId,
    /// The allowlisted paths within the repository.
    ///
    /// A prefix match is used: a grant for `src` also covers `src/main.rs`.
    pub allowed_paths: Vec<SafePath>,
}

impl PathGrant {
    /// Returns `true` if `path` is covered by this grant.
    ///
    /// A path is covered if it exactly matches an allowed path or if it is a
    /// descendant of an allowed directory prefix.
    pub fn allows(&self, path: &SafePath) -> bool {
        let target = path.as_str();
        self.allowed_paths.iter().any(|allowed| {
            let prefix = allowed.as_str();
            target == prefix || target.starts_with(&format!("{prefix}/"))
        })
    }
}
