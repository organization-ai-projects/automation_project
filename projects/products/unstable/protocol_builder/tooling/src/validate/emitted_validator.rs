// projects/products/unstable/protocol_builder/tooling/src/validate/emitted_validator.rs
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmittedManifest {
    pub manifest_hash: String,
    pub artifacts: BTreeMap<String, String>,
}

pub struct EmittedValidator;

impl EmittedValidator {
    /// Validates that the manifest_hash matches the artifact contents.
    pub fn validate(manifest: &EmittedManifest) -> Result<()> {
        let mut hasher = Sha256::new();
        for content in manifest.artifacts.values() {
            hasher.update(content.as_bytes());
        }
        let computed = hex::encode(hasher.finalize());
        if computed != manifest.manifest_hash {
            anyhow::bail!(
                "manifest_hash mismatch: expected {} got {}",
                manifest.manifest_hash,
                computed
            );
        }
        tracing::info!(manifest_hash = %computed, "emitted artifact validation passed");
        Ok(())
    }
}
