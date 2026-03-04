use serde::{Deserialize, Serialize};

/// A minimal issue entry as shown in the IDE issue list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueEntry {
    /// The opaque issue identifier.
    pub id: String,
    /// The display name for this issue.
    pub name: String,
    /// Optional description.
    pub description: Option<String>,
}
