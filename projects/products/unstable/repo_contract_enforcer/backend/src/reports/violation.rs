use crate::config::path_classification::PathClassification;
use crate::config::severity::Severity;
use crate::reports::violation_code::ViolationCode;
use crate::rules::rule_id::RuleId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Violation {
    pub rule_id: RuleId,
    pub violation_code: ViolationCode,
    pub severity: Severity,
    pub scope: PathClassification,
    pub path: String,
    pub message: String,
    pub line: Option<u32>,
}
