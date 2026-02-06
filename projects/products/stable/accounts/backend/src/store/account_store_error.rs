// projects/products/stable/accounts/backend/src/store/error.rs

#[derive(Debug, thiserror::Error)]
pub enum AccountStoreError {
    #[error("account not found")]
    NotFound,
    #[error("account already exists")]
    AlreadyExists,
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("invalid password")]
    InvalidPassword,
    #[error("invalid role")]
    InvalidRole,
    #[error("invalid permission")]
    InvalidPermission,
    #[error("invalid status")]
    InvalidStatus,
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(String),
    #[error("password error: {0}")]
    Password(String),
}
