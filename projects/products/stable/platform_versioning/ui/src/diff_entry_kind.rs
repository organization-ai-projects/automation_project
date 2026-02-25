// projects/products/stable/platform_versioning/ui/src/diff_entry_kind.rs
use serde::{Deserialize, Serialize};

/// The kind of change in a diff entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiffEntryKind {
    Added,
    Deleted,
    Modified,
}
