use thiserror::Error;

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("validation error: {0}")]
    Validation(String),
    #[error("codec error: {0}")]
    Codec(String),
    #[error("constraint error: {0}")]
    Constraint(String),
    #[error("replay error: {0}")]
    Replay(String),
    #[error("engine error: {0}")]
    Engine(String),
}
