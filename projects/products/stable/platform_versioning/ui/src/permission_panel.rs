// projects/products/stable/platform_versioning/ui/src/permission_panel.rs
use serde::{Deserialize, Serialize};

use crate::permission_entry::PermissionEntry;

/// Admin view for managing repository and global permissions.
///
/// Displays a table of current grants and allows admins to add or revoke
/// them. Developers see only their own grants.
///
/// This panel is not an IDE: it contains no file editor or terminal.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PermissionPanel {
    /// The permission entries currently loaded.
    pub entries: Vec<PermissionEntry>,
    /// Whether the panel is in read-only mode (i.e. the user is not an admin).
    pub read_only: bool,
}

impl PermissionPanel {
    /// Loads the given permission entries into the panel.
    pub fn load(&mut self, entries: Vec<PermissionEntry>, is_admin: bool) {
        self.entries = entries;
        self.read_only = !is_admin;
    }

    /// Adds a new permission entry (admin only; call is ignored in read-only mode).
    pub fn add_entry(&mut self, entry: PermissionEntry) {
        if !self.read_only {
            self.entries.push(entry);
        }
    }

    /// Removes the permission entry for the given subject + repo combination.
    ///
    /// Has no effect in read-only mode.
    pub fn remove_entry(&mut self, subject: &str, repo_id: &str) {
        if !self.read_only {
            self.entries
                .retain(|e| !(e.subject == subject && e.repo_id == repo_id));
        }
    }
}
