use thiserror::Error;

#[derive(Debug, Error)]
pub enum ErrorAnalyzerError {
    #[error("Analysis failed: {0}")]
    AnalysisFailed(String),
}