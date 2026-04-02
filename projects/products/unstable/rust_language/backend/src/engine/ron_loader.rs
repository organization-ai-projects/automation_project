//! projects/products/unstable/rust_language/backend/src/engine/ron_loader.rs
use crate::engine::engine_errors::EngineErrors;
use crate::model::ProjectConfig;

use std::path::Path;

pub(crate) struct RonLoader;

impl RonLoader {
    pub(crate) fn load_config(path: &Path) -> Result<ProjectConfig, EngineErrors> {
        common_ron::read_ron(path).map_err(|e| EngineErrors::Runtime(e.to_string()))
    }

    pub(crate) fn save_config(path: &Path, config: &ProjectConfig) -> Result<(), EngineErrors> {
        common_ron::write_ron(path, config).map_err(|e| EngineErrors::Runtime(e.to_string()))
    }
}
