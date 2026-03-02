// projects/products/unstable/simulation_compiler/tooling/src/validate/golden_validator.rs
use crate::diagnostics::error::ToolingError;
use sha2::{Digest, Sha256};

pub struct GoldenValidationResult {
    pub matched: bool,
}

pub struct GoldenValidator;

impl GoldenValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn check(
        &self,
        pack_dir: &str,
        golden_dir: &str,
    ) -> Result<GoldenValidationResult, ToolingError> {
        if pack_dir.is_empty() || golden_dir.is_empty() {
            return Ok(GoldenValidationResult { matched: true });
        }
        let pack_hash = hash_dir(pack_dir)?;
        let golden_hash = hash_dir(golden_dir)?;
        if pack_hash != golden_hash {
            return Err(ToolingError::GoldenMismatch(format!(
                "pack hash {pack_hash} != golden hash {golden_hash}"
            )));
        }
        Ok(GoldenValidationResult { matched: true })
    }
}

fn hash_dir(dir: &str) -> Result<String, ToolingError> {
    let path = std::path::Path::new(dir);
    if !path.exists() {
        return Err(ToolingError::Io(format!("directory not found: {dir}")));
    }
    let mut entries: Vec<_> = std::fs::read_dir(path)
        .map_err(|e| ToolingError::Io(e.to_string()))?
        .filter_map(|e| e.ok())
        .collect();
    entries.sort_by_key(|e| e.path());

    let mut hasher = Sha256::new();
    for entry in entries {
        let content = std::fs::read(entry.path()).map_err(|e| ToolingError::Io(e.to_string()))?;
        hasher.update(entry.path().to_string_lossy().as_bytes());
        hasher.update(b"\0");
        hasher.update(&content);
        hasher.update(b"\0");
    }
    Ok(hex::encode(hasher.finalize()))
}
