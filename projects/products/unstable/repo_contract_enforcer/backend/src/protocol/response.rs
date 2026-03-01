use crate::report::report::Report;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub id: Option<String>,
    #[serde(flatten)]
    pub payload: ResponsePayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ResponsePayload {
    Ok,
    Error {
        code: String,
        message: String,
        details: Option<String>,
    },
    Report {
        report_json: Report,
        report_hash: String,
        summary: crate::report::report::ReportSummary,
    },
}
