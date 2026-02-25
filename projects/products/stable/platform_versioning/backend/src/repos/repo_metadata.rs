// projects/products/stable/platform_versioning/backend/src/repos/repo_metadata.rs
use serde::{Deserialize, Serialize};

use crate::ids::RepoId;

/// Mutable metadata for a repository.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepoMetadata {
    /// The repository identifier.
    pub id: RepoId,
    /// The display name of the repository.
    pub name: String,
    /// An optional description.
    pub description: Option<String>,
    /// Unix timestamp when the repository was created.
    pub created_at: u64,
    /// Unix timestamp of the last metadata update.
    pub updated_at: u64,
}
