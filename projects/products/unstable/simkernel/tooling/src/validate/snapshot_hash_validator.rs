#![allow(dead_code)]
use crate::diagnostics::error::ToolingError;

pub struct SnapshotHashValidator;
impl SnapshotHashValidator {
    pub fn validate_hash(hash: &str) -> Result<(), ToolingError> {
        if hash.len() != 64 {
            return Err(ToolingError::ContractViolation(format!(
                "Invalid snapshot hash length: expected 64, got {}",
                hash.len()
            )));
        }
        if !hash.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(ToolingError::ContractViolation(
                "Non-hex chars in snapshot hash".to_string(),
            ));
        }
        Ok(())
    }
}
