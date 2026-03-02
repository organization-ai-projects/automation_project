// projects/products/unstable/protocol_builder/backend/src/output/manifest_hash.rs
use sha2::{Digest, Sha256};

use super::artifact_manifest::ArtifactManifest;

/// Computes manifest_hash = hex(SHA-256(sorted concatenation of all artifact bytes)).
pub fn compute_manifest_hash(manifest: &ArtifactManifest) -> String {
    let mut hasher = Sha256::new();
    // BTreeMap iteration is already in sorted key order
    for content in manifest.artifacts.values() {
        hasher.update(content.as_bytes());
    }
    let result = hasher.finalize();
    hex::encode(result)
}
