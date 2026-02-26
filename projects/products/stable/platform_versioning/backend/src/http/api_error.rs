// projects/products/stable/platform_versioning/backend/src/http/api_error.rs
use serde::{Deserialize, Serialize};

use crate::errors::{ErrorCategory, ErrorCode, PvError};

/// A machine-readable HTTP API error.
///
/// Serialized as the `error` field inside a [`super::ResponseEnvelope`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiError {
    /// Stable error code.
    pub code: ErrorCode,
    /// Stable error category.
    pub category: ErrorCategory,
    /// Human-readable error message (not stable; for debugging only).
    pub message: String,
}

impl From<PvError> for ApiError {
    fn from(err: PvError) -> Self {
        Self {
            code: err.code(),
            category: err.category(),
            message: err.to_string(),
        }
    }
}

/// Maps a [`PvError`] to the appropriate HTTP status code.
pub fn http_status_for(err: &PvError) -> u16 {
    use crate::errors::ErrorCategory;
    match err.category() {
        ErrorCategory::InvalidInput => 400,
        ErrorCategory::NotFound => 404,
        ErrorCategory::Integrity => 422,
        ErrorCategory::Io => 503,
        ErrorCategory::Conflict => 409,
        ErrorCategory::Unauthenticated => 401,
        ErrorCategory::Unauthorized => 403,
        ErrorCategory::Internal => 500,
    }
}
