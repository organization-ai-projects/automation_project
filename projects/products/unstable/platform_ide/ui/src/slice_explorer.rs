// projects/products/unstable/platform_ide/ui/src/slice_explorer.rs
use serde::{Deserialize, Serialize};

/// A single entry in the slice file explorer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SliceEntry {
    /// The allowed file path (validated by the backend before reaching the UI).
    pub path: String,
    /// Whether this file has local unsaved changes.
    pub dirty: bool,
}

/// The slice file explorer view state.
///
/// Displays only the paths from the slice manifest — no forbidden paths
/// are ever loaded into this view.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SliceExplorer {
    /// The issue identifier this slice belongs to.
    pub issue_id: Option<String>,
    /// The allowed file entries.
    pub entries: Vec<SliceEntry>,
}

impl SliceExplorer {
    /// Loads the slice for the given issue.
    pub fn load(&mut self, issue_id: String, paths: Vec<String>) {
        self.issue_id = Some(issue_id);
        self.entries = paths
            .into_iter()
            .map(|p| SliceEntry {
                path: p,
                dirty: false,
            })
            .collect();
    }

    /// Marks a path as dirty (has unsaved changes).
    pub fn mark_dirty(&mut self, path: &str) {
        for e in &mut self.entries {
            if e.path == path {
                e.dirty = true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_sets_entries() {
        let mut explorer = SliceExplorer::default();
        explorer.load(
            "issue-42".to_string(),
            vec!["src/main.rs".to_string(), "README.md".to_string()],
        );
        assert_eq!(explorer.issue_id.as_deref(), Some("issue-42"));
        assert_eq!(explorer.entries.len(), 2);
        assert!(!explorer.entries[0].dirty);
    }

    #[test]
    fn mark_dirty_sets_flag() {
        let mut explorer = SliceExplorer::default();
        explorer.load("issue-42".to_string(), vec!["src/main.rs".to_string()]);
        explorer.mark_dirty("src/main.rs");
        assert!(explorer.entries[0].dirty);
    }
}
