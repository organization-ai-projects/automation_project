use serde::{Deserialize, Serialize};

/// Canonical UI state snapshot for export/import.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiSnapshot {
    pub version: u32,
    pub state_json: String,
    pub checksum: String,
}

impl UiSnapshot {
    pub fn new(state_json: String, checksum: String) -> Self {
        Self {
            version: 1,
            state_json,
            checksum,
        }
    }
}
