// projects/libraries/layers/domain/security/src/token_error.rs
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum TokenError {
    #[error("invalid duration")]
    InvalidDuration,

    #[error("invalid subject id format")]
    InvalidSubjectIdFormat,

    #[error("invalid subject id value")]
    InvalidSubjectIdValue,

    #[error("timestamp overflow")]
    TimestampOverflow,

    #[error("invalid session id")]
    InvalidSessionId,

    #[error("missing secret")]
    MissingSecret,

    #[error("secret too short (min 32 chars)")]
    SecretTooShort,

    #[error("jwt error: {0}")]
    Jwt(String),

    #[error("token expired")]
    Expired,

    #[error("invalid token")]
    InvalidToken,

    #[error("cannot renew expired token")]
    CannotRenewExpired,
}
