use crate::diagnostics::AnalyzerError;

/// Loads source code from the given file path.
pub fn load_source(path: &str) -> Result<String, AnalyzerError> {
    std::fs::read_to_string(path).map_err(|e| AnalyzerError::InvalidSource(format!("{path}: {e}")))
}
