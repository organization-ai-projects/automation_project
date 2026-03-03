use serde::{Deserialize, Serialize};

/// A verification finding entry for display.
///
/// Only safe summaries are shown; forbidden paths are never presented.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindingEntry {
    /// "error", "warning", or "info".
    pub severity: String,
    /// Safe human-readable summary.
    pub summary: String,
    /// Optional allowed file path (absent for contract-level/global findings).
    pub path: Option<String>,
    /// Optional line number (absent when path is not present).
    pub line: Option<u32>,
}
