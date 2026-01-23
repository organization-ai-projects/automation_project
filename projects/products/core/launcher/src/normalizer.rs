// projects/products/core/launcher/src/normalizer.rs
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NormalizerError {
    #[error("Failed to normalize path")]
    NormalizePathError,
}

impl From<std::io::Error> for NormalizerError {
    fn from(_: std::io::Error) -> Self {
        NormalizerError::NormalizePathError
    }
}

pub fn normalize_path(p: &Path) -> Result<PathBuf, NormalizerError> {
    let abs = if p.is_absolute() {
        p.to_path_buf()
    } else {
        std::env::current_dir()?.join(p)
    };
    Ok(abs)
}
