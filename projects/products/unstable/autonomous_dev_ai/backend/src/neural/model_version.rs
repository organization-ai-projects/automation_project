//! projects/products/unstable/autonomous_dev_ai/src/neural/model_version.rs
use serde::{Deserialize, Serialize};

use crate::models::Version;

/// A pinned model version entry in the registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelVersion {
    pub name: String,
    pub version: Version,
    pub path: String,
    /// Minimum confidence score this model must produce to be trusted.
    pub confidence_threshold: f64,
    /// Whether the model is currently active in production.
    pub active: bool,
}

impl ModelVersion {
    pub fn new(
        name: impl Into<String>,
        version: Version,
        path: impl Into<String>,
        confidence_threshold: f64,
    ) -> Self {
        Self {
            name: name.into(),
            version,
            path: path.into(),
            confidence_threshold,
            active: false,
        }
    }
}
