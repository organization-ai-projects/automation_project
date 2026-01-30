// projects/products/core/launcher/src/normalizer.rs
use anyhow::Result;
use std::path::{Path, PathBuf};

pub(crate) fn normalize_path(p: &Path) -> Result<PathBuf, anyhow::Error> {
    let abs = if p.is_absolute() {
        p.to_path_buf()
    } else {
        std::env::current_dir()?.join(p)
    };
    Ok(abs)
}
