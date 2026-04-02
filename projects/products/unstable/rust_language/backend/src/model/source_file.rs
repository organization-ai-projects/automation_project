//! projects/products/unstable/rust_language/backend/src/model/source_file.rs
use std::path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct SourceFile {
    pub(crate) path: String,
    pub(crate) content: String,
}

impl SourceFile {
    pub(crate) fn new(path: String, content: String) -> Self {
        Self { path, content }
    }

    pub(crate) fn extension(&self) -> Option<&str> {
        path::Path::new(&self.path)
            .extension()
            .and_then(|ext| ext.to_str())
    }

    pub(crate) fn is_rhl(&self) -> bool {
        self.extension() == Some("rhl")
    }
}
