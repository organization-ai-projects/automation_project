use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    #[error("missing argument: {0}")]
    MissingArgument(String),

    #[error("run error: {0}")]
    RunError(String),

    #[error("replay error: {0}")]
    ReplayError(String),

    #[error("export error: {0}")]
    ExportError(String),
}
