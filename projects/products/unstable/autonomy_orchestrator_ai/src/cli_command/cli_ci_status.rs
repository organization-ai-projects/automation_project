// projects/products/unstable/autonomy_orchestrator_ai/src/cli_command/cli_ci_status.rs
use crate::domain::CiGateStatus;
use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum CliCiStatus {
    Success,
    Pending,
    Failure,
    Missing,
}

impl From<CliCiStatus> for CiGateStatus {
    fn from(value: CliCiStatus) -> Self {
        match value {
            CliCiStatus::Success => Self::Success,
            CliCiStatus::Pending => Self::Pending,
            CliCiStatus::Failure => Self::Failure,
            CliCiStatus::Missing => Self::Missing,
        }
    }
}
