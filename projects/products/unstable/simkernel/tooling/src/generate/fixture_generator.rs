#![allow(dead_code)]
use crate::diagnostics::error::ToolingError;
use std::path::Path;

pub struct FixtureGenerator;
impl FixtureGenerator {
    pub fn generate_golden(pack_kind: &str, out_dir: &Path) -> Result<(), ToolingError> {
        std::fs::create_dir_all(out_dir).map_err(|e| ToolingError::Io(e.to_string()))?;
        let placeholder = serde_json::json!({
            "pack_kind": pack_kind,
            "run_hash": "placeholder",
            "note": "replace with actual golden output"
        });
        let data = serde_json::to_string_pretty(&placeholder).map_err(|e| ToolingError::Serialization(e.to_string()))?;
        let fname = format!("{}_golden.json", pack_kind);
        std::fs::write(out_dir.join(&fname), data).map_err(|e| ToolingError::Io(e.to_string()))?;
        Ok(())
    }
}
