//! projects/products/unstable/rust_language/backend/src/model/project_config.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ProjectConfig {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) entry_point: String,
    pub(crate) source_files: Vec<String>,
}

impl ProjectConfig {
    pub(crate) fn new(name: String, version: String, entry_point: String) -> Self {
        Self {
            name,
            version,
            entry_point,
            source_files: Vec::new(),
        }
    }

    /// Creates a default configuration with common values.
    pub(crate) fn default_config(file_path: impl Into<String>) -> Self {
        ProjectConfig::new("inline".into(), "0.1.0".into(), file_path.into())
    }
}
