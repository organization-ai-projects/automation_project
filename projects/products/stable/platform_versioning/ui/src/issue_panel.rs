// projects/products/stable/platform_versioning/ui/src/issue_panel.rs
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
