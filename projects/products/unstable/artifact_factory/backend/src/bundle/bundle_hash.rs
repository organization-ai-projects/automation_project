use crate::bundle::artifact_bundle::ArtifactBundle;
use sha2::{Digest, Sha256};

/// Derives a deterministic SHA-256 hash for a bundle.
/// Hash input = manifest entries (sorted) + file bytes (in manifest order).
pub struct BundleHash;

impl BundleHash {
    pub fn compute(bundle: &ArtifactBundle) -> String {
        let mut hasher = Sha256::new();
        // Feed manifest entries in sorted order
        for name in &bundle.manifest {
            hasher.update(name.as_bytes());
            hasher.update(b"\x00");
            if let Some(bytes) = bundle.files.get(name) {
                hasher.update(bytes);
            }
            hasher.update(b"\x01");
        }
        hex::encode(hasher.finalize())
    }
}
