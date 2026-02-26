// projects/products/unstable/autonomy_orchestrator_ai/src/cli_command/cli_review_status.rs
use crate::domain::ReviewGateStatus;
use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum CliReviewStatus {
    Approved,
    #[value(name = "changes_requested")]
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
