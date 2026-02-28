// projects/products/unstable/code_forge_engine/tooling/src/golden/golden_report.rs
use std::path::PathBuf;
use crate::diagnostics::error::ToolingError;

pub struct GoldenReport {
    pub dir: PathBuf,
}

pub struct GoldenCheckResult {
    pub all_passed: bool,
    pub failures: Vec<String>,
}

impl GoldenReport {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self { dir: dir.into() }
    }

    pub fn check(&self) -> Result<GoldenCheckResult, ToolingError> {
        Ok(GoldenCheckResult { all_passed: true, failures: vec![] })
    }
}
