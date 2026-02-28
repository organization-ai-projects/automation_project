// projects/products/unstable/code_forge_engine/backend/src/render/path_layout.rs
use std::path::{Path, PathBuf};

pub struct PathLayout {
    root: PathBuf,
}

impl PathLayout {
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self { root: root.as_ref().to_path_buf() }
    }

    pub fn resolve(&self, relative: &str) -> PathBuf {
        self.root.join(relative)
    }

    pub fn sorted_paths(paths: &mut Vec<String>) {
        paths.sort();
    }
}
