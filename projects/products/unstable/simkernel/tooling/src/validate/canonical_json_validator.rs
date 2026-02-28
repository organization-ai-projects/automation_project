#![allow(dead_code)]
use crate::diagnostics::error::ToolingError;

pub struct CanonicalJsonValidator;
impl CanonicalJsonValidator {
    pub fn validate(json: &str) -> Result<(), ToolingError> {
        let v: serde_json::Value = serde_json::from_str(json)
            .map_err(|e| ToolingError::ContractViolation(format!("Invalid JSON: {}", e)))?;
        let _ =
            serde_json::to_string(&v).map_err(|e| ToolingError::Serialization(e.to_string()))?;
        Ok(())
    }
}
