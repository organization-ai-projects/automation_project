use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub entry_point: String,
    pub source_files: Vec<String>,
}

impl ProjectConfig {
    pub fn new(name: String, version: String, entry_point: String) -> Self {
        Self {
            name,
            version,
            entry_point,
            source_files: Vec::new(),
        }
    }
}
