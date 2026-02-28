// projects/products/unstable/code_forge_engine/tooling/src/golden/golden_updater.rs
use std::path::PathBuf;
use crate::diagnostics::error::ToolingError;

pub struct GoldenUpdater {
    pub dir: PathBuf,
}

impl GoldenUpdater {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self { dir: dir.into() }
    }

    pub fn update(&self) -> Result<(), ToolingError> {
        std::fs::create_dir_all(&self.dir)
            .map_err(|e| ToolingError::Io(e.to_string()))?;
        Ok(())
    }
}
