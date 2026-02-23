// projects/products/unstable/platform_versioning/ui/src/issue_panel.rs
use serde::{Deserialize, Serialize};

use crate::issue_summary::IssueSummary;

/// Admin view for managing issue assignment and sharing.
///
/// Admins see all issues and can assign developers or share issues.
/// Developers see only the issues that are assigned to them or shared with
/// them—no other issues are visible. This panel is not an IDE.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct IssuePanel {
    /// Issues loaded for the current user's visibility scope.
    pub issues: Vec<IssueSummary>,
    /// Whether the panel allows write operations (admin only).
    pub read_only: bool,
    /// The currently selected issue id, if any.
    pub selected_id: Option<String>,
}

impl IssuePanel {
    /// Loads the visible issue list for the current user.
    ///
    /// `is_admin` controls whether write actions (assign/share) are available.
    pub fn load(&mut self, issues: Vec<IssueSummary>, is_admin: bool) {
        self.issues = issues;
        self.read_only = !is_admin;
        self.selected_id = None;
    }

    /// Selects the issue with the given id.
    pub fn select(&mut self, id: String) {
        self.selected_id = Some(id);
    }

    /// Deselects the current issue.
    pub fn deselect(&mut self) {
        self.selected_id = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn summary(id: &str) -> IssueSummary {
        IssueSummary {
            id: id.to_string(),
            title: "Test Issue".to_string(),
            repo_id: None,
            assignee_count: 0,
        }
    }

    #[test]
    fn admin_load_shows_all() {
        let mut panel = IssuePanel::default();
        panel.load(vec![summary("iss-1"), summary("iss-2")], true);
        assert_eq!(panel.issues.len(), 2);
        assert!(!panel.read_only);
    }

    #[test]
    fn dev_load_is_read_only() {
        let mut panel = IssuePanel::default();
        panel.load(vec![summary("iss-1")], false);
        assert!(panel.read_only);
    }

    #[test]
    fn select_sets_selected_id() {
        let mut panel = IssuePanel::default();
        panel.load(vec![summary("iss-1")], false);
        panel.select("iss-1".to_string());
        assert_eq!(panel.selected_id.as_deref(), Some("iss-1"));
    }

    #[test]
    fn deselect_clears_selection() {
        let mut panel = IssuePanel::default();
        panel.select("iss-1".to_string());
        panel.deselect();
        assert!(panel.selected_id.is_none());
    }
}
