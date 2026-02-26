// projects/products/unstable/autonomy_orchestrator_ai/src/domain/policy_gate_status.rs

use serde::{Deserialize, Serialize};

use crate::cli_command::CliPolicyStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyGateStatus {
    Allow,
    Deny,
    Unknown,
}

impl From<CliPolicyStatus> for PolicyGateStatus {
    fn from(value: CliPolicyStatus) -> Self {
        match value {
            CliPolicyStatus::Allow => Self::Allow,
            CliPolicyStatus::Deny => Self::Deny,
            CliPolicyStatus::Unknown => Self::Unknown,
        }
    }
}
