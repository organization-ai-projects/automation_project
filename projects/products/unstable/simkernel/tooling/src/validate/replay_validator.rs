#![allow(dead_code)]
use crate::diagnostics::error::ToolingError;
use std::path::Path;

pub struct ReplayValidator;
impl ReplayValidator {
    pub fn validate(path: &Path) -> Result<(), ToolingError> {
        if !path.exists() {
            return Err(ToolingError::ContractViolation(format!("Replay file not found: {}", path.display())));
        }
        let content = std::fs::read_to_string(path).map_err(|e| ToolingError::Io(e.to_string()))?;
        let v: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| ToolingError::ContractViolation(format!("Invalid replay JSON: {}", e)))?;
        if v.get("pack_id").is_none() {
            return Err(ToolingError::ContractViolation("Missing pack_id in replay".to_string()));
        }
        Ok(())
    }
}
