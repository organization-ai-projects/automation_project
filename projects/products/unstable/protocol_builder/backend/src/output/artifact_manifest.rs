// projects/products/unstable/protocol_builder/backend/src/output/artifact_manifest.rs
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A manifest of emitted artifacts keyed by artifact name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactManifest {
    pub artifacts: BTreeMap<String, String>,
}

impl ArtifactManifest {
    pub fn new() -> Self {
        Self {
            artifacts: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, name: impl Into<String>, content: impl Into<String>) {
        self.artifacts.insert(name.into(), content.into());
    }
}

impl Default for ArtifactManifest {
    fn default() -> Self {
        Self::new()
    }
}
