// projects/products/unstable/platform_versioning/backend/src/issues/issue.rs
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
        self.assignees.iter().any(|a| a == subject)
            || self.shared_with.iter().any(|s| s == subject)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::slices::SliceDefinition;

    fn make_issue() -> Issue {
        Issue {
            id: "issue-1".parse().unwrap(),
            repo_id: None,
            title: "Test Issue".to_string(),
            description: None,
            assignees: vec!["alice".to_string()],
            shared_with: vec!["bob".to_string()],
            slice_definition: None,
        }
    }

    #[test]
    fn assignee_is_visible() {
        let issue = make_issue();
        assert!(issue.is_visible_to("alice"));
    }

    #[test]
    fn shared_user_is_visible() {
        let issue = make_issue();
        assert!(issue.is_visible_to("bob"));
    }

    #[test]
    fn unrelated_user_is_not_visible() {
        let issue = make_issue();
        assert!(!issue.is_visible_to("mallory"));
    }

    #[test]
    fn slice_definition_stored_on_issue() {
        let def = SliceDefinition::from_paths(vec!["src".to_string()]).unwrap();
        let mut issue = make_issue();
        issue.slice_definition = Some(def.clone());
        assert_eq!(issue.slice_definition.as_ref().unwrap().paths().len(), 1);
    }
}
