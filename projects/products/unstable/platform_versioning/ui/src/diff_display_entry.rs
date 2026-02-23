// projects/products/unstable/platform_versioning/ui/src/diff_display_entry.rs
use serde::{Deserialize, Serialize};

use crate::diff_entry_kind::DiffEntryKind;

/// A single file diff entry for display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffDisplayEntry {
    /// The file path.
    pub path: String,
    /// The kind of change.
    pub kind: DiffEntryKind,
    /// Whether this is a binary file.
    pub binary: bool,
}
