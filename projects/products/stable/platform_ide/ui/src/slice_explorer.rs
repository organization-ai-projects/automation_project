// projects/products/stable/platform_ide/ui/src/slice_explorer.rs
use crate::slice_entry::SliceEntry;
use serde::{Deserialize, Serialize};

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
