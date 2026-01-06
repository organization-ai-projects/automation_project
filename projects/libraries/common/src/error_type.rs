use thiserror::Error;

#[derive(Debug, Error)]
pub enum ErrorType {
    #[error("Invalid ID provided")]
    InvalidID,

    #[error("Unknown error occurred")]
    Unknown,
}
