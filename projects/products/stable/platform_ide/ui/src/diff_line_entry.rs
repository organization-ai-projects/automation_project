use serde::{Deserialize, Serialize};

use crate::diff_line_kind::DiffLineKind;

/// A diff line for display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffLineEntry {
    pub kind: DiffLineKind,
    pub content: String,
}
