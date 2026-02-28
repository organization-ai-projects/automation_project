// projects/products/unstable/code_forge_engine/tooling/src/validate/structure_validator.rs
use crate::diagnostics::error::ToolingError;
use std::path::PathBuf;

pub struct StructureValidator {
    pub dir: PathBuf,
}

impl StructureValidator {
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
