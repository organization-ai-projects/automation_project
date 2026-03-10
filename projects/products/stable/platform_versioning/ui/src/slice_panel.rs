// projects/products/stable/platform_versioning/ui/src/slice_panel.rs
use serde::{Deserialize, Serialize};

/// Admin view for managing slice definitions attached to issues.
///
/// An admin can view and set the path allowlist for any issue. A developer
/// can only view the slice definition for issues they are assigned to or
/// that have been shared with them. This panel is not an IDE.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SlicePanel {
    /// The issue id whose slice definition is being viewed or edited.
    pub issue_id: Option<String>,
    /// The current allowed paths in the slice definition.
    pub allowed_paths: Vec<String>,
    /// Whether this panel is in read-only mode (i.e. the user is not an admin).
    pub read_only: bool,
}

impl SlicePanel {
    /// Loads the slice definition for an issue.
    ///
    /// `is_admin` controls whether the paths can be edited.
    pub fn load(&mut self, issue_id: String, paths: Vec<String>, is_admin: bool) {
        self.issue_id = Some(issue_id);
        self.allowed_paths = paths;
        self.read_only = !is_admin;
    }

    /// Adds a path to the allowlist (admin only; ignored in read-only mode).
    pub fn add_path(&mut self, path: String) {
        if !self.read_only && !self.allowed_paths.contains(&path) {
            self.allowed_paths.push(path);
            self.allowed_paths.sort();
        }
    }

    /// Removes a path from the allowlist (admin only; ignored in read-only mode).
    pub fn remove_path(&mut self, path: &str) {
        if !self.read_only {
            self.allowed_paths.retain(|p| p != path);
        }
    }
}
