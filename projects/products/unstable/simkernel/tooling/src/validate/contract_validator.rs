#![allow(dead_code)]
use crate::diagnostics::error::ToolingError;
use std::path::Path;

pub fn validate_contract(path: &Path) -> Result<(), ToolingError> {
    if !path.exists() {
        return Err(ToolingError::ContractViolation(format!(
            "File not found: {}",
            path.display()
        )));
    }
    let content = std::fs::read_to_string(path).map_err(|e| ToolingError::Io(e.to_string()))?;
    serde_json::from_str::<serde_json::Value>(&content)
        .map_err(|e| ToolingError::ContractViolation(format!("Invalid JSON: {}", e)))?;
    Ok(())
}
