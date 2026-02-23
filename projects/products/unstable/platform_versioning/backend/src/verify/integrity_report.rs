// projects/products/unstable/platform_versioning/backend/src/verify/integrity_report.rs
use serde::{Deserialize, Serialize};

use crate::verify::IntegrityIssue;

/// Summary report of a repository integrity check.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegrityReport {
    /// All detected issues, or empty if the repository is healthy.
    pub issues: Vec<IntegrityIssue>,
    /// Number of objects checked.
    pub objects_checked: usize,
    /// Number of refs checked.
    pub refs_checked: usize,
}

impl IntegrityReport {
    /// Returns `true` if no issues were found.
    pub fn is_healthy(&self) -> bool {
        self.issues.is_empty()
    }
}
