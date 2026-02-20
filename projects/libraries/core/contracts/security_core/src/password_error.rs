use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum PasswordError {
    #[error("password hash error: {0}")]
    HashError(String),
}
