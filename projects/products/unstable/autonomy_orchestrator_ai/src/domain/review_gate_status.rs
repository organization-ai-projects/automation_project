// projects/products/unstable/autonomy_orchestrator_ai/src/domain/review_gate_status.rs

use serde::{Deserialize, Serialize};

use crate::cli_command::CliReviewStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewGateStatus {
    Approved,
    ChangesRequested,
    Missing,
}

impl From<CliReviewStatus> for ReviewGateStatus {
    fn from(value: CliReviewStatus) -> Self {
        match value {
            CliReviewStatus::Approved => Self::Approved,
            CliReviewStatus::ChangesRequested => Self::ChangesRequested,
            CliReviewStatus::Missing => Self::Missing,
        }
    }
}
