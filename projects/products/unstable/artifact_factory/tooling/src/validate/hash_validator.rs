use crate::diagnostics::error::ToolingError;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::Path;

pub struct HashValidator;

impl HashValidator {
    /// Compute the canonical hash for a bundle directory.
    /// Uses the same algorithm as BundleHash in backend.
    pub fn compute_hash(bundle_dir: &Path) -> Result<String, ToolingError> {
        // Collect all files sorted deterministically
        let mut files: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        let entries = std::fs::read_dir(bundle_dir)
            .map_err(|e| ToolingError::Io(e.to_string()))?;
        for entry in entries {
            let entry = entry.map_err(|e| ToolingError::Io(e.to_string()))?;
            let name = entry.file_name().to_string_lossy().to_string();
            let bytes = std::fs::read(entry.path()).map_err(|e| ToolingError::Io(e.to_string()))?;
            files.insert(name, bytes);
        }

        let mut hasher = Sha256::new();
        for (name, bytes) in &files {
            hasher.update(name.as_bytes());
            hasher.update(b"\x00");
            hasher.update(bytes);
            hasher.update(b"\x01");
        }
        Ok(hex::encode(hasher.finalize()))
    }

    /// Verify that `bundle_dir` hashes to `expected_hash`.
    pub fn verify(bundle_dir: &Path, expected_hash: &str) -> Result<(), ToolingError> {
        let actual = Self::compute_hash(bundle_dir)?;
        if actual != expected_hash {
            return Err(ToolingError::HashMismatch {
                expected: expected_hash.to_string(),
                actual,
            });
        }
        tracing::info!(hash = %actual, "hash verification passed");
        Ok(())
    }
}
