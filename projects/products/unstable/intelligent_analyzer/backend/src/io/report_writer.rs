use crate::diagnostics::AnalyzerError;
use crate::report::AnalysisReport;

/// Writes an analysis report as JSON to the given file path.
pub fn write_json_report(report: &AnalysisReport, path: &str) -> Result<(), AnalyzerError> {
    let json =
        common_json::to_string_pretty(report).map_err(|e| AnalyzerError::Io(e.to_string()))?;
    std::fs::write(path, json).map_err(|e| AnalyzerError::Io(e.to_string()))
}
