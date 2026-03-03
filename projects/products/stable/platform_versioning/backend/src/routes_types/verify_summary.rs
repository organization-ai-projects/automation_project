use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct VerifySummary {
    pub healthy: bool,
    pub report: crate::verify::IntegrityReport,
}
