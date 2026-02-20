//! AiBody is the only public interface of the `ai` library.
//! Do not use directly: ai_orchestrator.rs and ai_feedback.rs.
// projects/libraries/layers/orchestration/ai/src/ai_body.rs

use std::path::Path;
use tracing::warn;

use crate::{
    ai_error::AiError, ai_orchestrator::AiOrchestrator,
    feedbacks::public_api_feedback::FeedbackInput, solver_strategy::SolverStrategy, task::Task,
    task_result::TaskResult,
};

pub struct AiBody {
    orchestrator: AiOrchestrator,
}

impl AiBody {
    pub fn new() -> Result<Self, AiError> {
        Ok(Self {
            orchestrator: AiOrchestrator::new()?,
        })
    }

    pub fn load_neural_model(
        &mut self,
        model_path: &Path,
        tokenizer_path: &Path,
    ) -> Result<bool, AiError> {
        if model_path.is_file() && tokenizer_path.is_file() {
            self.orchestrator
                .load_neural_model(model_path, tokenizer_path)?;
            Ok(true)
        } else {
            warn!(
                "Model or tokenizer file not found or not a file. Skipping neural model loading."
            );
            Ok(false)
        }
    }

    pub fn solve(&mut self, task: &Task) -> Result<TaskResult, AiError> {
        self.orchestrator.solve(task)
    }

    pub fn solve_with_strategy(
        &mut self,
        task: &Task,
        strategy: SolverStrategy,
    ) -> Result<TaskResult, AiError> {
        self.orchestrator.solve_forced(task, strategy)
    }

    pub fn solve_symbolic_then_neural(&mut self, task: &Task) -> Result<TaskResult, AiError> {
        self.solve_with_strategy(task, SolverStrategy::SymbolicThenNeural)
    }

    pub fn solve_neural_with_validation(&mut self, task: &Task) -> Result<TaskResult, AiError> {
        self.solve_with_strategy(task, SolverStrategy::NeuralWithSymbolicValidation)
    }

    pub fn solve_hybrid(&mut self, task: &Task) -> Result<TaskResult, AiError> {
        self.solve_with_strategy(task, SolverStrategy::Hybrid)
    }

    // --- Feedback public API (primitives, not exposing internal types) ---

    /// Train (neural + symbolic) via simple verdict.
    /// ok=true => Correct, ok=false => Incorrect
    pub fn train_with_verdict(
        &mut self,
        task: &Task,
        input: &str,
        generated_output: &str,
        ok: bool,
    ) -> Result<(), AiError> {
        self.orchestrator
            .train_with_verdict(task, input, generated_output, ok)
    }

    /// Adjust the system with detailed feedback (legacy / best-effort).
    ///
    /// This does NOT carry solve-path information. For neurosymbolic "path-aware"
    /// training, prefer `adjust_with_result(...)` (uses result.trace()).
    pub fn adjust(&mut self, req: FeedbackInput<'_>) -> Result<(), AiError> {
        self.orchestrator.adjust(&req)
    }

    /// Adjust the system with detailed feedback, using the `TaskResult` solve trace.
    ///
    /// This enables path-aware neurosymbolic training (winner/fallback/validation/hybrid).
    ///
    /// Usage pattern:
    /// 1) let result = ai.solve(&task)?;
    /// 2) ai.adjust_with_result(feedback, &result)?;
    pub fn adjust_with_result(
        &mut self,
        req: FeedbackInput<'_>,
        result: &TaskResult,
    ) -> Result<(), AiError> {
        self.orchestrator.adjust_with_trace(&req, result.trace())
    }

    // --- Simplified API (unchanged on usage side) ---

    pub fn generate_code(&mut self, prompt: &str) -> Result<String, AiError> {
        self.orchestrator.generate_code(prompt)
    }

    pub fn analyze_code(&mut self, code: &str) -> Result<String, AiError> {
        self.orchestrator.analyze_code(code)
    }

    pub fn refactor_code(&mut self, code: &str, instruction: &str) -> Result<String, AiError> {
        self.orchestrator.refactor_code(code, instruction)
    }

    pub fn train_neural(
        &mut self,
        training_data: impl IntoIterator<Item = String>,
        model_path: &Path,
    ) -> Result<(), AiError> {
        let data: Vec<String> = training_data.into_iter().collect();
        self.orchestrator.train_neural(data, model_path)
    }

    pub fn evaluate_model(
        &mut self,
        evaluate_data: impl IntoIterator<Item = String>,
    ) -> Result<f64, AiError> {
        self.orchestrator.evaluate_model(evaluate_data)
    }

    pub fn save_neural_model(
        &self,
        model_path: &Path,
        tokenizer_path: &Path,
    ) -> Result<(), AiError> {
        self.orchestrator
            .save_neural_model(model_path, tokenizer_path)
    }

    pub fn append_training_example(
        &self,
        replay_path: &Path,
        example_json: &str,
    ) -> Result<(), AiError> {
        self.orchestrator
            .append_training_example_json(replay_path, example_json)
    }

    pub fn load_training_examples(&self, replay_path: &Path) -> Result<Vec<String>, AiError> {
        self.orchestrator
            .load_training_examples_as_strings(replay_path)
    }

    /// Create a task from a prompt.
    pub fn create_task(&self, prompt: &str) -> Task {
        Task::new_code_generation(prompt.to_string())
    }
}
