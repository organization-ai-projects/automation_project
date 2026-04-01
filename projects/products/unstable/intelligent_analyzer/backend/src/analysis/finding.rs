use serde::{Deserialize, Serialize};

use super::finding_kind::FindingKind;
use super::severity::Severity;

/// A single finding produced by the analysis or linting engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub kind: FindingKind,
    pub severity: Severity,
    pub line: usize,
    pub message: String,
}

impl Finding {
    pub fn new(kind: FindingKind, severity: Severity, line: usize, message: String) -> Self {
        Self {
            kind,
            severity,
            line,
            message,
        }
    }
}
