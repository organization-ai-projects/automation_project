// projects/products/stable/platform_versioning/backend/src/http/response_envelope.rs
use serde::{Deserialize, Serialize};

use crate::http::ApiError;

/// The standard HTTP response envelope for all platform-versioning API endpoints.
///
/// On success, `data` is `Some` and `error` is `None`.
/// On error, `data` is `None` and `error` is `Some`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponseEnvelope<T> {
    /// Whether the request succeeded.
    pub ok: bool,
    /// The response payload (present on success).
    pub data: Option<T>,
    /// The error details (present on failure).
    pub error: Option<ApiError>,
}

impl<T> ResponseEnvelope<T> {
    /// Creates a successful envelope.
    pub fn ok(data: T) -> Self {
        Self {
            ok: true,
            data: Some(data),
            error: None,
        }
    }

    /// Creates an error envelope.
    pub fn err(error: ApiError) -> Self {
        Self {
            ok: false,
            data: None,
            error: Some(error),
        }
    }
}
