use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    Io(String),
    #[error("json error: {0}")]
    Json(String),
    #[error("validation error: {0}")]
    Validation(String),
    #[error("migration error: {0}")]
    Migration(String),
}
