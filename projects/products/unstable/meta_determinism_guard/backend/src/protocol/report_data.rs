use crate::stability::stability_report::StabilityReport;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportData {
    pub scan_findings: Vec<String>,
    pub canon_issues: Vec<String>,
    pub stability: Option<StabilityReport>,
}
