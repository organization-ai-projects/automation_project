// projects/products/unstable/platform_versioning/ui/src/issue_summary.rs
use serde::{Deserialize, Serialize};

/// A compact summary of an issue used in list views.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IssueSummary {
    /// Unique identifier for this issue.
    pub id: String,
    /// Short human-readable title.
    pub title: String,
    /// The repository this issue is scoped to, if any.
    pub repo_id: Option<String>,
    /// Number of users assigned to this issue.
    pub assignee_count: usize,
}
