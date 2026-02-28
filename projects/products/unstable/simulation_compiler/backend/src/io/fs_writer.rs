// projects/products/unstable/simulation_compiler/backend/src/io/fs_writer.rs
use crate::diagnostics::error::CompilerError;
use std::path::Path;

pub struct FsWriter {
    base: String,
}

impl FsWriter {
    pub fn new(base: impl Into<String>) -> Self {
        Self { base: base.into() }
    }

    pub fn write(&self, rel_path: &str, content: &[u8]) -> Result<(), CompilerError> {
        let full_path = Path::new(&self.base).join(rel_path);
        if let Some(parent) = full_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| CompilerError::Io(e.to_string()))?;
        }
        std::fs::write(&full_path, content)
            .map_err(|e| CompilerError::Io(e.to_string()))
    }
}
