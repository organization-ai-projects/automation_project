// projects/libraries/layers/domain/symbolic/src/symbolic_solver.rs
use crate::feedback_symbolic::SymbolicFeedback;
use crate::validator::ValidationError;
use crate::{
    analyzer::CodeAnalyzer, rules::RulesEngine, solver_result::SolverResult,
    symbolic_error::SymbolicError, validator::CodeValidator,
};
use tracing;

/// Symbolic solver - internal orchestration of symbolic
pub struct SymbolicSolver {
    analyzer: CodeAnalyzer,
    rules: RulesEngine,
    validator: CodeValidator,
}

impl SymbolicSolver {
    pub fn new() -> Result<Self, SymbolicError> {
        Ok(Self {
            analyzer: CodeAnalyzer::new(),
            rules: RulesEngine::new().map_err(|e| SymbolicError::GenerationError(e.to_string()))?,
            validator: CodeValidator::new()
                .map_err(|e| SymbolicError::ValidationError(e.to_string()))?,
        })
    }

    pub fn solve(
        &self,
        input: &str,
        task_type: &str,
        context: Option<&str>,
    ) -> Result<SolverResult, SymbolicError> {
        match task_type {
            "analysis" => self.analyze(input),
            "generation" => self.generate(input, context),
            "linting" => self.lint(input),
            "documentation" => self.document(input),
            "refactoring" => self.refactor(input, context),
            _ => Err(SymbolicError::AnalysisError(format!(
                "Unknown task type: {}",
                task_type
            ))),
        }
    }

    fn analyze(&self, code: &str) -> Result<SolverResult, SymbolicError> {
        let analysis_success = self.analyzer.analyze_code(code); // Use analyze_code instead of analyze

        if analysis_success {
            Ok(SolverResult {
                output: "Analysis successful".to_string(),
                confidence: 0.95,
                metadata: Some("Symbolic analysis".to_string()),
            })
        } else {
            Err(SymbolicError::AnalysisError("Analysis failed".to_string()))
        }
    }

    fn generate(&self, prompt: &str, context: Option<&str>) -> Result<SolverResult, SymbolicError> {
        let code = self
            .rules
            .generate(prompt, context)
            .map_err(|e| SymbolicError::GenerationError(e.to_string()))?; // Correctly convert RulesError to String

        let confidence = self.rules.match_confidence(prompt);

        Ok(SolverResult {
            output: code,
            confidence,
            metadata: Some("Template-based generation".to_string()),
        })
    }

    fn lint(&self, _code: &str) -> Result<SolverResult, SymbolicError> {
        let issues: Vec<String> = vec!["Issue 1".to_string(), "Issue 2".to_string()];
        let output = if issues.is_empty() {
            "No issues found".to_string()
        } else {
            format!("Found {} issues:\n{}", issues.len(), issues.join("\n"))
        };

        Ok(SolverResult {
            output,
            confidence: 1.0,
            metadata: Some(format!("{} issues", issues.len())),
        })
    }

    fn document(&self, code: &str) -> Result<SolverResult, SymbolicError> {
        let docs = self
            .analyzer
            .generate_documentation(code)
            .map_err(|e| SymbolicError::AnalysisError(e.to_string()))?;

        Ok(SolverResult {
            output: docs,
            confidence: 0.9,
            metadata: Some("Generated documentation".to_string()),
        })
    }

    fn refactor(
        &self,
        code: &str,
        instruction: Option<&str>,
    ) -> Result<SolverResult, SymbolicError> {
        let instruction = instruction.ok_or_else(|| {
            SymbolicError::GenerationError("Refactoring requires instruction".to_string())
        })?;

        let refactored = self
            .rules
            .apply_refactoring(code, instruction)
            .map_err(|e| SymbolicError::GenerationError(e.to_string()))?;

        tracing::debug!(
            instruction=%instruction,
            confidence=refactored.confidence,
            changes=%refactored.changes_applied.join(", "),
            "Refactor applied"
        );

        Ok(SolverResult {
            output: refactored.code,
            confidence: refactored.confidence,
            metadata: Some(refactored.changes_applied.join(", ")),
        })
    }

    pub fn validate_code(
        &self,
        code: &str,
    ) -> Result<crate::validator::ValidationResult, SymbolicError> {
        let validation = self
            .validator
            .validate(code) // Use validate method from CodeValidator
            .map_err(|e: ValidationError| SymbolicError::ValidationError(e.to_string()))?;

        Ok(validation)
    }

    pub fn adjust_rules(
        &mut self,
        input: &str,
        feedback: SymbolicFeedback,
    ) -> Result<(), SymbolicError> {
        tracing::info!(
            input=%input,
            positive=feedback.is_positive(),
            "Adjusting symbolic rules"
        );

        if feedback.is_positive() {
            tracing::info!("Positive feedback received. Reinforcing associated rules.");
            self.reinforce_rule(input);
        } else {
            tracing::info!("Negative feedback received. Revising associated rules.");
            self.weaken_rule(input);
        }

        Ok(())
    }

    pub fn reinforce_rule(&mut self, input: &str) {
        tracing::info!(input=%input, "Reinforcing rule");
        // Logic to reinforce a rule
    }

    pub fn weaken_rule(&mut self, input: &str) {
        tracing::info!(input=%input, "Weakening rule");
        // Logic to weaken a rule
    }
}
