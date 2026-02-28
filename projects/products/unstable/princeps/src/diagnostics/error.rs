use thiserror::Error;

#[derive(Debug, Error)]
pub enum PrincepsError {
    #[error("simulation failed: {0}")]
    Simulation(String),
    #[error("replay failed: {0}")]
    Replay(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serialization error: {0}")]
    Serialization(String),
    #[error("no candidates registered")]
    NoCandidates,
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
}
