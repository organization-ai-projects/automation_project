use ai::{AiBody, AiError, SolverStrategy, Task, TaskResult};

use super::insight::Insight;
use super::insight_kind::InsightKind;
use crate::diagnostics::AnalyzerError;

/// Neurosymbolic engine that delegates to the `ai` orchestration library.
pub struct NeurosymbolicEngine {
    ai: AiBody,
}

impl NeurosymbolicEngine {
    pub fn new() -> Result<Self, AnalyzerError> {
        let ai = AiBody::new().map_err(|e: AiError| AnalyzerError::Neurosymbolic(e.to_string()))?;
        Ok(Self { ai })
    }

    /// Analyse source code using the hybrid neuro-symbolic strategy.
    pub fn analyze(&mut self, source: &str) -> Result<Vec<Insight>, AnalyzerError> {
        let task = Task::new_code_analysis(source.to_string());
        let result = self
            .ai
            .solve_with_strategy(&task, SolverStrategy::Hybrid)
            .map_err(|e| AnalyzerError::Neurosymbolic(e.to_string()))?;

        Ok(Self::result_to_insights(&result))
    }

    /// Generate a refactoring suggestion for the given source code.
    pub fn suggest_refactoring(
        &mut self,
        source: &str,
        instruction: &str,
    ) -> Result<Vec<Insight>, AnalyzerError> {
        let task = Task::new_refactoring(source.to_string(), instruction.to_string());
        let result = self
            .ai
            .solve_with_strategy(&task, SolverStrategy::SymbolicThenNeural)
            .map_err(|e| AnalyzerError::Neurosymbolic(e.to_string()))?;

        Ok(Self::result_to_insights(&result))
    }

    fn result_to_insights(result: &TaskResult) -> Vec<Insight> {
        let kind = if result.output.contains("refactor") {
            InsightKind::Refactoring
        } else if result.output.contains("pattern") {
            InsightKind::PatternDetection
        } else if result.output.contains("complex") {
            InsightKind::ComplexityWarning
        } else {
            InsightKind::Suggestion
        };

        vec![Insight::new(
            kind,
            result.confidence,
            result.output.clone(),
            format!("{:?}", result.strategy_used),
        )]
    }
}
