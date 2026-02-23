// projects/products/unstable/platform_versioning/backend/src/http/api_version.rs
use serde::{Deserialize, Serialize};

/// Explicit API version discriminator.
///
/// # Versioning strategy
/// All HTTP endpoints are prefixed with `/v{N}/` where N is the numeric version.
/// New versions are added when breaking changes are required. Old versions remain
/// available for a documented deprecation window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApiVersion {
    /// The initial stable API version.
    V1,
}

impl ApiVersion {
    /// Returns the URL path prefix for this version (e.g. `"/v1"`).
    pub fn path_prefix(&self) -> &'static str {
        match self {
            Self::V1 => "/v1",
        }
    }
}
