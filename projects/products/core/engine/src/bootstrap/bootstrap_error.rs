// projects/products/core/engine/src/bootstrap/bootstrap_error.rs

/// Errors during bootstrap/setup operations
#[derive(Debug, thiserror::Error)]
pub enum BootstrapError {
    #[error("home directory is not available")]
    HomeDirUnavailable,

    #[error("claim file missing")]
    ClaimMissing,

    #[error("claim expired")]
    ClaimExpired,

    #[error("claim invalid")]
    ClaimInvalid,

    #[error("setup already completed")]
    SetupAlreadyCompleted,

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(String),
}
