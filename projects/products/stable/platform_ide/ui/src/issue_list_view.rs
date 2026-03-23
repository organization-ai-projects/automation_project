// projects/products/stable/platform_ide/ui/src/issue_list_view.rs
use crate::issue_entry::IssueEntry;
use serde::{Deserialize, Serialize};

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
