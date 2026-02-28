// projects/products/unstable/simulation_compiler/tooling/src/validate/emitted_pack_validator.rs
use crate::diagnostics::error::ToolingError;

pub struct PackValidationResult {
    pub valid: bool,
    pub file_count: usize,
}

pub struct EmittedPackValidator;

impl EmittedPackValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_dir(&self, dir: &str) -> Result<PackValidationResult, ToolingError> {
        if dir.is_empty() {
            return Ok(PackValidationResult { valid: true, file_count: 0 });
        }
        let path = std::path::Path::new(dir);
        if !path.exists() {
            return Err(ToolingError::Validation(format!(
                "pack directory does not exist: {dir}"
            )));
        }
        let file_count = std::fs::read_dir(path)
            .map_err(|e| ToolingError::Io(e.to_string()))?
            .count();
        Ok(PackValidationResult { valid: true, file_count })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_dir_returns_valid() {
        let v = EmittedPackValidator::new();
        let result = v.validate_dir("").unwrap();
        assert!(result.valid);
        assert_eq!(result.file_count, 0);
    }
}
