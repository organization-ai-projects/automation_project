// projects/products/unstable/autonomy_orchestrator_ai/src/cli_command/cli_reviews_status.rs
use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum CliReviewStatus {
    Approved,
    #[value(name = "changes_requested")]
    ChangesRequested,
    Missing,
}
