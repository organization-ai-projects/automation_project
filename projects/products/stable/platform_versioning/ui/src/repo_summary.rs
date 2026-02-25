// projects/products/stable/platform_versioning/ui/src/repo_summary.rs
use serde::{Deserialize, Serialize};

/// A repository summary for display in the list view.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoSummary {
    /// The repository id.
    pub id: String,
    /// The display name.
    pub name: String,
    /// Optional description.
    pub description: Option<String>,
}
