// projects/products/unstable/platform_ide/ui/src/issue_list_view.rs
use serde::{Deserialize, Serialize};

/// A minimal issue entry as shown in the IDE issue list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueEntry {
    /// The opaque issue identifier.
    pub id: String,
    /// The display name for this issue.
    pub name: String,
    /// Optional description.
    pub description: Option<String>,
}

/// The issue list view state.
///
/// Shows only issues that the platform made visible to the current user.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct IssueListView {
    /// The currently loaded list of visible issues.
    pub issues: Vec<IssueEntry>,
    /// Whether a fetch is in progress.
    pub loading: bool,
}

impl IssueListView {
    /// Updates the issue list with fresh data.
    pub fn set_issues(&mut self, issues: Vec<IssueEntry>) {
        self.issues = issues;
        self.loading = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_issues_updates_list() {
        let mut view = IssueListView {
            loading: true,
            ..IssueListView::default()
        };
        view.set_issues(vec![IssueEntry {
            id: "issue-1".to_string(),
            name: "My Issue".to_string(),
            description: None,
        }]);
        assert_eq!(view.issues.len(), 1);
        assert!(!view.loading);
    }
}
