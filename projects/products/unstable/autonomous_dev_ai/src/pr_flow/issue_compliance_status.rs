// projects/products/unstable/autonomous_dev_ai/src/pr_flow/issue_compliance_status.rs
use serde::{Deserialize, Serialize};

/// Compliance state relative to repository rules.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IssueComplianceStatus {
    Compliant,
    NonCompliant { reason: String },
    Unknown,
}
