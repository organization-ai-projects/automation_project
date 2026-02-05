// projects/products/core/launcher/src/workspace.rs
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Workspace {
    pub(crate) root: PathBuf,
}
