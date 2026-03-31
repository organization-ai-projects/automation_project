use serde::{Deserialize, Serialize};

/// Severity level for analysis findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Severity {
    Hint,
    Warning,
    Error,
}
