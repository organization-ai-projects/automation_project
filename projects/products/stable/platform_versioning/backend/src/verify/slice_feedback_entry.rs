use serde::{Deserialize, Serialize};

/// A single feedback entry for an integrity issue within an allowed path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SliceFeedbackEntry {
    /// Human-readable summary of the issue (safe to display to the caller).
    pub summary: String,
}
