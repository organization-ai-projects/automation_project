use serde::{Deserialize, Serialize};

use crate::verification::FindingSeverity;

/// A raw finding as received from the platform API before filtering.
#[derive(Debug, Serialize, Deserialize)]
pub struct RawFinding {
    pub severity: FindingSeverity,
    pub summary: String,
    pub path: Option<String>,
    pub line: Option<u32>,
}
