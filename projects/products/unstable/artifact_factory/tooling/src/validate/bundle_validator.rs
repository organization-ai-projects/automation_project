use crate::diagnostics::error::ToolingError;
use std::path::Path;

pub struct BundleValidator;

impl BundleValidator {
    /// Validate that a bundle directory contains the expected files.
    pub fn validate(bundle_dir: &Path, expected_files: &[&str]) -> Result<(), ToolingError> {
        for file in expected_files {
            let path = bundle_dir.join(file);
            if !path.exists() {
                return Err(ToolingError::Validation(format!(
                    "missing expected file: {file}"
                )));
            }
        }
        tracing::info!(
            dir = %bundle_dir.display(),
            checked = expected_files.len(),
            "bundle validation passed"
        );
        Ok(())
    }
}
