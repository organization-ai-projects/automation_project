use thiserror::Error;

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("validation error: {0}")]
    Validation(String),
    #[error("codec error: {0}")]
    Codec(String),
    #[error("data error: {0}")]
    Data(String),
    #[error("engine error: {0}")]
    Engine(String),
    #[error("capture error: {0}")]
    Capture(String),
    #[error("combat error: {0}")]
    Combat(String),
    #[error("replay error: {0}")]
    Replay(String),
    #[error("scenario error: {0}")]
    Scenario(String),
}
