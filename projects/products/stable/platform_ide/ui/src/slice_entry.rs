use serde::{Deserialize, Serialize};

/// A single entry in the slice file explorer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SliceEntry {
    /// The allowed file path (validated by the backend before reaching the UI).
    pub path: String,
    /// Whether this file has local unsaved changes.
    pub dirty: bool,
}
