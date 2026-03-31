use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("serialization error: {0}")]
    Serialization(String),

    #[error("end of input")]
    EndOfInput,

    #[error("analysis failed: {0}")]
    #[allow(dead_code)]
    AnalysisFailed(String),

    #[error("invalid argument: {0}")]
    #[allow(dead_code)]
    InvalidArgument(String),

    #[error("expert failure: {0}")]
    #[allow(dead_code)]
    ExpertFailure(String),
}
