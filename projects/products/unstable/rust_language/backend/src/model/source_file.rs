use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceFile {
    pub path: String,
    pub content: String,
}

impl SourceFile {
    pub fn new(path: String, content: String) -> Self {
        Self { path, content }
    }

    pub fn extension(&self) -> Option<&str> {
        std::path::Path::new(&self.path)
            .extension()
            .and_then(|ext| ext.to_str())
    }

    pub fn is_rhl(&self) -> bool {
        self.extension() == Some("rhl")
    }
}
