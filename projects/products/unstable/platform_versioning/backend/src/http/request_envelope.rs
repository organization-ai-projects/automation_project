// projects/products/unstable/platform_versioning/backend/src/http/request_envelope.rs
use serde::{Deserialize, Serialize};

use crate::http::ApiVersion;

/// Metadata attached to every incoming HTTP request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestEnvelope {
    /// The API version the client is targeting.
    pub version: ApiVersion,
    /// The bearer token provided by the client (if any).
    pub token: Option<String>,
}
