#![allow(dead_code)]
use crate::diagnostics::error::SimError;
use crate::report::run_report::RunReport;

pub struct ReportCodec;

impl ReportCodec {
    pub fn encode_canonical(report: &RunReport) -> Result<String, SimError> {
        serde_json::to_string(report).map_err(|e| SimError::Serialization(e.to_string()))
    }

    pub fn decode(data: &str) -> Result<RunReport, SimError> {
        serde_json::from_str(data).map_err(|e| SimError::Serialization(e.to_string()))
    }
}
