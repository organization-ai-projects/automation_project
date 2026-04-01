use serde::{Deserialize, Serialize};

use crate::analysis::Finding;
use crate::neurosymbolic::Insight;

/// Aggregated result produced by the analysis pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineResult {
    pub analysis_findings: Vec<Finding>,
    pub lint_findings: Vec<Finding>,
    pub insights: Vec<Insight>,
    pub source_hash: String,
}
