// projects/products/unstable/autonomy_orchestrator_ai/src/cli_command/cli_policy_status.rs
use crate::domain::PolicyGateStatus;
use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum CliPolicyStatus {
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
