// projects/products/unstable/code_forge_engine/backend/src/output/artifact_manifest.rs
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactManifest {
    pub name: String,
    pub files: BTreeMap<String, Vec<u8>>,
}

impl ArtifactManifest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            files: BTreeMap::new(),
        }
    }

    pub fn add_file(&mut self, path: impl Into<String>, bytes: Vec<u8>) {
        self.files.insert(path.into(), bytes);
    }

    pub fn sorted_paths(&self) -> Vec<&str> {
        self.files.keys().map(|s| s.as_str()).collect()
    }
}
