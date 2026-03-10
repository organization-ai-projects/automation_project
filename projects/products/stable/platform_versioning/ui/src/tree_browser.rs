// projects/products/stable/platform_versioning/ui/src/tree_browser.rs
use serde::{Deserialize, Serialize};

use crate::tree_browser_entry::TreeBrowserEntry;

/// The tree browser view state.
///
/// Allows navigating the directory tree of a specific commit.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TreeBrowser {
    /// The current path being browsed (empty = root).
    pub current_path: String,
    /// The entries in the current directory.
    pub entries: Vec<TreeBrowserEntry>,
}

impl TreeBrowser {
    /// Navigates into a sub-directory.
    pub fn navigate(&mut self, dir_name: &str, entries: Vec<TreeBrowserEntry>) {
        if self.current_path.is_empty() {
            self.current_path = dir_name.to_string();
        } else {
            self.current_path = format!("{}/{}", self.current_path, dir_name);
        }
        self.entries = entries;
    }

    /// Navigates to the parent directory.
    pub fn go_up(&mut self, entries: Vec<TreeBrowserEntry>) {
        if let Some(pos) = self.current_path.rfind('/') {
            self.current_path = self.current_path[..pos].to_string();
        } else {
            self.current_path.clear();
        }
        self.entries = entries;
    }
}
