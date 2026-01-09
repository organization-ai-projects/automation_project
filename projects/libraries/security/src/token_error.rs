use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum TokenError {
    #[error("invalid duration")]
    InvalidDuration,

    #[error("invalid user id format")]
    InvalidUserIdFormat,

    #[error("invalid user id value")]
    InvalidUserIdValue,

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
