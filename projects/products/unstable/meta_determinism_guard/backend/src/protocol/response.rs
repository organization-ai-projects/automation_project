use crate::stability::stability_report::StabilityReport;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Response {
    Ok,
    Error { message: String },
    Report { data: ReportData },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportData {
    pub scan_findings: Vec<String>,
    pub canon_issues: Vec<String>,
    pub stability: Option<StabilityReport>,
}
