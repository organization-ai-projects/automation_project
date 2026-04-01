use sha2::{Digest, Sha256};

use super::PipelineResult;
use crate::analysis::CodeAnalysis;
use crate::config::AnalyzerConfig;
use crate::diagnostics::AnalyzerError;
use crate::linting::LintEngine;
use crate::neurosymbolic::NeurosymbolicEngine;

/// Runs the full analysis pipeline: structural analysis, linting, and
/// neurosymbolic AI insights.
pub fn run_pipeline(
    config: &AnalyzerConfig,
    source: &str,
) -> Result<PipelineResult, AnalyzerError> {
    let source_hash = {
        let mut hasher = Sha256::new();
        hasher.update(source.as_bytes());
        hex::encode(hasher.finalize())
    };

    let analysis_findings = if config.enable_analysis {
        CodeAnalysis::analyze(source)
    } else {
        Vec::new()
    };

    let lint_findings = if config.enable_linting {
        LintEngine::lint(source)
    } else {
        Vec::new()
    };

    let insights = if config.enable_neurosymbolic {
        let mut engine = NeurosymbolicEngine::new()?;
        engine.analyze(source)?
    } else {
        Vec::new()
    };

    Ok(PipelineResult {
        analysis_findings,
        lint_findings,
        insights,
        source_hash,
    })
}
