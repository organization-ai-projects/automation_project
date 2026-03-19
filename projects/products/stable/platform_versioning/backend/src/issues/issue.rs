// projects/products/stable/platform_versioning/backend/src/issues/issue.rs
use serde::{Deserialize, Serialize};

use crate::ids::RepoId;
use crate::issues::IssueId;
use crate::slices::SliceDefinition;

/// A tracked work item scoped to a repository.
///
/// Issues form the primary unit of task-scoped access: developers only see
/// issues that are assigned to them or explicitly shared, and the
/// [`SliceDefinition`] attached to an issue defines which repository paths
/// they may access while working on it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Issue {
    /// Unique identifier for this issue.
    pub id: IssueId,
    /// The repository this issue belongs to, if any.
    pub repo_id: Option<RepoId>,
    /// Short human-readable title.
    pub title: String,
    /// Optional longer description.
    pub description: Option<String>,
    /// Subjects (user identifiers) that are assigned to work on this issue.
    pub assignees: Vec<String>,
    /// Subjects that have read-only visibility into this issue (but are not
    /// assignees).
    pub shared_with: Vec<String>,
    /// Path allowlist defining which repository paths are accessible through
    /// this issue. When `None` no path restriction is applied via this issue.
    pub slice_definition: Option<SliceDefinition>,
}

impl Issue {
    /// Returns `true` if `subject` is assigned to this issue or has been
    /// explicitly shared access.
    pub fn is_visible_to(&self, subject: &str) -> bool {
        self.assignees.iter().any(|a| a == subject) || self.shared_with.iter().any(|s| s == subject)
    }
}
