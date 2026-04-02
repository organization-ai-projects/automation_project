use crate::cli::output_mode::OutputMode;

#[derive(Debug, Clone)]
pub struct ReplayArgs {
    pub replay_file: String,
    pub output_mode: OutputMode,
}
