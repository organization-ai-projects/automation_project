// projects/products/unstable/protocol_builder/backend/src/output/generate_report.rs
use serde::{Deserialize, Serialize};

use super::artifact_manifest::ArtifactManifest;
use super::manifest_hash::compute_manifest_hash;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateReport {
    pub manifest_hash: String,
    pub artifact_names: Vec<String>,
    pub artifact_count: usize,
}

impl GenerateReport {
    pub fn from_manifest(manifest: &ArtifactManifest) -> Self {
        let manifest_hash = compute_manifest_hash(manifest);
        let artifact_names: Vec<String> = manifest.artifacts.keys().cloned().collect();
        let artifact_count = artifact_names.len();
        Self {
            manifest_hash,
            artifact_names,
            artifact_count,
        }
    }
}
