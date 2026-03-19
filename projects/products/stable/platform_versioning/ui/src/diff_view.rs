// projects/products/stable/platform_versioning/ui/src/diff_view.rs
use serde::{Deserialize, Serialize};

use crate::diff_display_entry::DiffDisplayEntry;

/// The diff view state (read-only).
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DiffView {
    /// The base commit id.
    pub from_commit: Option<String>,
    /// The target commit id.
    pub to_commit: Option<String>,
    /// The diff entries to display.
    pub entries: Vec<DiffDisplayEntry>,
}

impl DiffView {
    /// Loads a diff result for display.
    pub fn load(&mut self, from: String, to: String, entries: Vec<DiffDisplayEntry>) {
        self.from_commit = Some(from);
        self.to_commit = Some(to);
        self.entries = entries;
    }
}
