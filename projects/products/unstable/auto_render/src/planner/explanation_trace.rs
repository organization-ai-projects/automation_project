use serde::{Deserialize, Serialize};
use super::ConstraintReport;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationTrace {
    pub summary: String,
    pub key_decisions: Vec<String>,
    pub constraint_report: ConstraintReport,
}
