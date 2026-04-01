use serde::{Deserialize, Serialize};

use crate::analysis::Finding;
use crate::neurosymbolic::Insight;
use crate::pipeline::PipelineResult;

/// Final report emitted by the analyzer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisReport {
    pub source_hash: String,
    pub total_findings: usize,
    pub total_insights: usize,
    pub findings: Vec<Finding>,
    pub insights: Vec<Insight>,
}

impl AnalysisReport {
    pub fn from_result(result: &PipelineResult) -> Self {
        let mut findings = result.analysis_findings.clone();
        findings.extend(result.lint_findings.clone());

        Self {
            source_hash: result.source_hash.clone(),
            total_findings: findings.len(),
            total_insights: result.insights.len(),
            findings,
            insights: result.insights.clone(),
        }
    }
}
