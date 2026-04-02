use serde::Serialize;

#[derive(Debug, thiserror::Error, Serialize)]
pub enum AnalyzerError {
    #[error("invalid source: {0}")]
    InvalidSource(String),

    #[error("pipeline failure: {0}")]
    PipelineFailure(String),

    #[error("neurosymbolic error: {0}")]
    Neurosymbolic(String),

    #[error("io error: {0}")]
    Io(String),
}
