// projects/products/unstable/code_forge_engine/backend/src/io/fs_writer.rs
use std::path::Path;
use crate::diagnostics::error::ForgeError;

pub struct FsWriter;

impl FsWriter {
    pub fn write(path: impl AsRef<Path>, bytes: &[u8]) -> Result<(), ForgeError> {
        if let Some(parent) = path.as_ref().parent() {
            std::fs::create_dir_all(parent).map_err(|e| ForgeError::Io(e.to_string()))?;
        }
        std::fs::write(path, bytes).map_err(|e| ForgeError::Io(e.to_string()))
    }
}
