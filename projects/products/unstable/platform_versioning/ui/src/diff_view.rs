// projects/products/unstable/platform_versioning/ui/src/diff_view.rs
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff_entry_kind::DiffEntryKind;

    #[test]
    fn load_sets_fields() {
        let mut view = DiffView::default();
        view.load(
            "abc".to_string(),
            "def".to_string(),
            vec![DiffDisplayEntry {
                path: "readme.md".to_string(),
                kind: DiffEntryKind::Modified,
                binary: false,
            }],
        );
        assert_eq!(view.from_commit.as_deref(), Some("abc"));
        assert_eq!(view.entries.len(), 1);
    }
}
