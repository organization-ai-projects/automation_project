// projects/products/unstable/code_forge_engine/tooling/src/validate/byte_stability_validator.rs
use crate::diagnostics::error::ToolingError;
use std::path::PathBuf;

pub struct ByteStabilityValidator {
    pub dir: PathBuf,
}

impl ByteStabilityValidator {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self { dir: dir.into() }
    }

    pub fn validate(&self) -> Result<(), ToolingError> {
        if !self.dir.exists() {
            return Ok(());
        }
        Ok(())
    }
}
