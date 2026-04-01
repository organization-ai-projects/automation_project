use serde::{Deserialize, Serialize};

/// Top-level configuration for the intelligent analyzer pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzerConfig {
    pub enable_analysis: bool,
    pub enable_linting: bool,
    pub enable_neurosymbolic: bool,
    pub confidence_threshold: f64,
}

impl Default for AnalyzerConfig {
    fn default() -> Self {
        Self {
            enable_analysis: true,
            enable_linting: true,
            enable_neurosymbolic: true,
            confidence_threshold: 0.6,
        }
    }
}
