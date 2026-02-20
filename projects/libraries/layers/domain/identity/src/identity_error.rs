// projects/libraries/layers/domain/identity/src/identity_error.rs
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum IdentityError {
    #[error("invalid user id format")]
    InvalidUserIdFormat,

    #[error("invalid user id value")]
    InvalidUserIdValue,

    #[error("empty password")]
    EmptyPassword,

    #[error("password hash error: {0}")]
    PasswordHashError(String),

    #[error("invalid credentials")]
    InvalidCredentials,
}
