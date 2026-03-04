use crate::diagnostics::backend_error::BackendError;
use std::path::Path;

pub struct FsWriter;

impl FsWriter {
    pub fn write(path: impl AsRef<Path>, bytes: &[u8]) -> Result<(), BackendError> {
        if let Some(parent) = path.as_ref().parent() {
            std::fs::create_dir_all(parent).map_err(|error| BackendError::Io(error.to_string()))?;
        }
        std::fs::write(path, bytes).map_err(|error| BackendError::Io(error.to_string()))
    }
}
