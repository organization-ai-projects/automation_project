// projects/products/stable/platform_ide/ui/src/diff_view.rs
use crate::diff_line_entry::DiffLineEntry;
use crate::diff_line_kind::DiffLineKind;
use serde::{Deserialize, Serialize};

/// The local diff view state for a single file.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DiffView {
    /// The file path being diffed (always an allowed path).
    pub path: Option<String>,
    /// The diff lines to display.
    pub lines: Vec<DiffLineEntry>,
}

impl DiffView {
    /// Loads the diff for a file.
    pub fn load(&mut self, path: String, lines: Vec<DiffLineEntry>) {
        self.path = Some(path);
        self.lines = lines;
    }

    /// Returns `true` if there are any added or removed lines.
    pub fn has_changes(&self) -> bool {
        self.lines.iter().any(|l| l.kind != DiffLineKind::Context)
    }
}
