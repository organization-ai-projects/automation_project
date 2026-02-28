#![allow(dead_code)]
use crate::diagnostics::error::ToolingError;
use std::path::Path;

pub struct ScenarioGenerator;
impl ScenarioGenerator {
    pub fn generate(pack_kind: &str, out_path: &Path) -> Result<(), ToolingError> {
        let scenario = serde_json::json!({
            "id": format!("{}_generated", pack_kind),
            "pack_kind": pack_kind,
            "seed": 42,
            "ticks": 10,
            "turns": 0,
            "ticks_per_turn": 10,
            "description": format!("Generated scenario for {}", pack_kind)
        });
        let data = serde_json::to_string_pretty(&scenario).map_err(|e| ToolingError::Serialization(e.to_string()))?;
        std::fs::write(out_path, data).map_err(|e| ToolingError::Io(e.to_string()))?;
        Ok(())
    }
}
