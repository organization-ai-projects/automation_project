// projects/products/unstable/simulation_compiler/backend/src/output/artifact_manifest.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactManifest {
    pub entries: Vec<ManifestEntry>,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestEntry {
    pub path: String,
    pub size: usize,
    pub sha256: String,
}
