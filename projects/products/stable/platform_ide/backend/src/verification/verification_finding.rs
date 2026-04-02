//! projects/products/stable/platform_ide/backend/src/verification/verification_finding.rs
use serde::{Deserialize, Serialize};

use crate::verification::FindingSeverity;

/// A single finding from a verification run, pre-filtered for display.
///
/// Findings that reference forbidden paths are demoted to a generic
/// contract-level error with no path details exposed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationFinding {
    /// Severity of the finding.
    pub severity: FindingSeverity,
    /// A safe, human-readable summary of the finding.
    pub summary: String,
    /// The allowed file path this finding relates to, if any.
    /// `None` if the finding is not file-specific or if the path is forbidden.
    pub path: Option<String>,
    /// The line number within `path`, if relevant and path is allowed.
    pub line: Option<u32>,
}
