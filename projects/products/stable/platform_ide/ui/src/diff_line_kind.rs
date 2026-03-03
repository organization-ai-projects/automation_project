use serde::{Deserialize, Serialize};

/// A single diff line kind for display.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiffLineKind {
    Added,
    Removed,
    Context,
}
