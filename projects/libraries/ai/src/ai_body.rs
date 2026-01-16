//! AiBody is the only public interface of the `ai` library.
//! Do not use directly: ai_orchestrator.rs and ai_feedback.rs.
use std::path;

// projects/libraries/ai/src/ai_body.rs
use tracing::warn;

use crate::{
    ai_error::AiError, ai_orchestrator::AiOrchestrator, feedbacks::FeedbackInput,
    solver_strategy::SolverStrategy, task::Task, task_result::TaskResult,
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
        model_path: &path::Path,
        tokenizer_path: &path::Path,
    ) -> Result<(), AiError> {
        if model_path.is_file() && tokenizer_path.is_file() {
            self.orchestrator
                .load_neural_model(model_path, tokenizer_path)
        } else {
            warn!(
                "Model or tokenizer file not found or not a file. Skipping neural model loading."
            );
            Ok(())
        }
    }

    pub fn solve(&mut self, task: &Task) -> Result<TaskResult, AiError> {
        self.orchestrator.solve(task)
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

    /// Adjust the system with detailed feedback.
    ///
    /// Accepts a `FeedbackInput`, which encapsulates:
    /// - The task context and input
    /// - The generated output
    /// - A structured verdict (Correct, Incorrect, Partial, or Rejected)
    /// - Optional metadata (confidence, rationale, source)
    ///
    /// The feedback is used to adjust both neural and symbolic components
    /// of the AI system, improving future predictions.
    ///
    /// # Example
    /// ```ignore
    /// use ai::feedbacks::{FeedbackInput, FeedbackMeta};
    ///
    /// let feedback = FeedbackInput::correct("task", "input", "output")
    ///     .meta(FeedbackMeta::new().confidence(0.95));
    /// ai.adjust(feedback)?;
    /// ```
    pub fn adjust(&mut self, req: FeedbackInput<'_>) -> Result<(), AiError> {
        let event = req.to_internal();

        // Appel à l'orchestrateur avec les types internes
        self.orchestrator.adjust(&event)
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
        training_data: Vec<String>,
        model_path: &std::path::Path,
    ) -> Result<(), AiError> {
        self.orchestrator.train_neural(training_data, model_path)
    }

    pub fn solve_symbolic_then_neural(&mut self, task: &Task) -> Result<TaskResult, AiError> {
        self.orchestrator
            .solve_forced(task, SolverStrategy::SymbolicThenNeural)
    }

    pub fn solve_neural_with_validation(&mut self, task: &Task) -> Result<TaskResult, AiError> {
        self.orchestrator
            .solve_forced(task, SolverStrategy::NeuralWithSymbolicValidation)
    }

    pub fn solve_hybrid(&mut self, task: &Task) -> Result<TaskResult, AiError> {
        self.orchestrator.solve_forced(task, SolverStrategy::Hybrid)
    }

    pub fn evaluate_model(&mut self, test_data: Vec<String>) -> Result<f64, AiError> {
        self.orchestrator.evaluate_model(test_data)
    }

    pub fn save_neural_model(
        &self,
        model_path: &path::Path,
        tokenizer_path: &path::Path,
    ) -> Result<(), AiError> {
        self.orchestrator
            .save_neural_model(model_path, tokenizer_path)
    }

    pub fn append_training_example(
        &self,
        replay_path: &path::Path,
        example_json: &str,
    ) -> Result<(), AiError> {
        self.orchestrator
            .append_training_example_json(replay_path, example_json)
    }

    pub fn load_training_examples(&self, replay_path: &path::Path) -> Result<Vec<String>, AiError> {
        self.orchestrator
            .load_training_examples_as_strings(replay_path)
    }

    /// Create a task from a prompt.
    pub fn create_task(&self, prompt: &str) -> Task {
        Task::new_code_generation(prompt.to_string())
    }
}
