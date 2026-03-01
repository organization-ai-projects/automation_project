use crate::config::enforcement_mode::EnforcementMode;
use crate::report::violation::Violation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReportSummary {
    pub stable_error_count: u64,
    pub stable_warning_count: u64,
    pub unstable_error_count: u64,
    pub unstable_warning_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Report {
    pub repository_root: String,
    pub mode: EnforcementMode,
    pub violations: Vec<Violation>,
    pub summary: ReportSummary,
    pub report_hash: String,
}
