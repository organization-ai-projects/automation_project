use crate::report::report::Report;
use anyhow::Result;

pub struct JsonReportCodec;

impl JsonReportCodec {
    pub fn to_json(report: &Report) -> Result<String> {
        Ok(serde_json::to_string(report)?)
    }
}
