use crate::cli::output_mode::OutputMode;

#[derive(Debug, Clone)]
pub struct RunArgs {
    pub ticks: u64,
    pub seed: u64,
    pub dataset: Option<String>,
    pub output_mode: OutputMode,
}
