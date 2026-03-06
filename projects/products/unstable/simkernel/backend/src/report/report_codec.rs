use crate::diagnostics::backend_error::BackendError;
use crate::report::run_report::RunReport;

pub struct ReportCodec;

impl ReportCodec {
    pub fn encode_canonical(report: &RunReport) -> Result<String, BackendError> {
        common_json::to_string(report).map_err(|e| BackendError::Serialization(e.to_string()))
    }

    pub fn decode(data: &str) -> Result<RunReport, BackendError> {
        common_json::from_str(data).map_err(|e| BackendError::Serialization(e.to_string()))
    }
}
