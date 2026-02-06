// projects/products/core/launcher/src/entry/paths.rs
use std::path::PathBuf;

use crate::workspace::Workspace;

#[derive(Clone)]
pub(crate) struct Paths {
    pub(crate) workspace: Workspace,
    pub(crate) profile_dir: PathBuf,
}
