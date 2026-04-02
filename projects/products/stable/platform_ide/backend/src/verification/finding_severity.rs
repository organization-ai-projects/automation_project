//! projects/products/stable/platform_ide/backend/src/verification/finding_severity.rs
use serde::{Deserialize, Serialize};

/// The severity level of a verification finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingSeverity {
    /// An error that must be resolved before the change can be accepted.
    Error,
    /// A warning that should be reviewed.
    Warning,
    /// Informational note.
    Info,
}
