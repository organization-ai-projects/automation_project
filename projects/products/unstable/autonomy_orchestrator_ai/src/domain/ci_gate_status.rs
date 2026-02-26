// projects/products/unstable/autonomy_orchestrator_ai/src/domain/ci_gate_status.rs

use serde::{Deserialize, Serialize};

use crate::cli_command::CliCiStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CiGateStatus {
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
