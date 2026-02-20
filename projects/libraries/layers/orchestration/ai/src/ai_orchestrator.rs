// projects/libraries/layers/orchestration/ai/src/ai_orchestrator.rs
use std::path::Path;
use tracing::{debug, info};

use crate::{
    ai_error::AiError,
    dispatcher::Dispatcher,
    feedbacks::internal::internal_feedback_input::InternalFeedbackInput,
    feedbacks::{ai_feedback::AiFeedback, public_api_feedback::FeedbackInput},
    solve_trace::SolveTrace,
    task::Task,
    task_result::TaskResult,
};

/// Internal structure for AI orchestration.
/// Provides feedback and dispatching capabilities.
pub(crate) struct AiOrchestrator {
    pub(crate) feedback: AiFeedback,
    pub(crate) dispatcher: Dispatcher,
}

impl AiOrchestrator {
    pub(crate) fn new() -> Result<Self, AiError> {
        info!("Initializing AI orchestrator...");

        Ok(Self {
            feedback: AiFeedback::new()?,
            dispatcher: Dispatcher::new(),
        })
    }

    pub(crate) fn load_neural_model(
        &mut self,
        model_path: &Path,
        tokenizer_path: &Path,
    ) -> Result<(), AiError> {
        debug!("Loading neural model...");
        self.feedback
            .load_neural_model(model_path, tokenizer_path)?;
        debug!("Neural model loaded successfully");
        Ok(())
    }

    pub(crate) fn save_neural_model(
        &self,
        model_path: &Path,
        tokenizer_path: &Path,
    ) -> Result<(), AiError> {
        self.feedback.save_neural_model(model_path, tokenizer_path)
    }

    // --- Feedback API ---

    // The train_with_verdict and train_with_feedback_neural methods have been moved to training.rs.

    /// Adjust feedback for both symbolic and neural solvers.
    /// This is a convenience wrapper around AiFeedback::adjust.
    /// Only accepts primitives.
    pub(crate) fn adjust(&mut self, feedback_input: &FeedbackInput<'_>) -> Result<(), AiError> {
        let internal_feedback_input = InternalFeedbackInput::from(feedback_input);
        self.feedback.adjust(&internal_feedback_input)
    }

    /// Adjust feedback using a solve trace (path-aware neurosymbolic training).
    pub(crate) fn adjust_with_trace(
        &mut self,
        feedback_input: &FeedbackInput<'_>,
        trace: &SolveTrace,
    ) -> Result<(), AiError> {
        let internal_feedback_input = InternalFeedbackInput::from(feedback_input);
        self.feedback
            .adjust_with_trace(&internal_feedback_input, trace)
    }

    // Simplified API
    pub(crate) fn generate_code(&mut self, prompt: &str) -> Result<String, AiError> {
        let task = Task::new_code_generation(prompt.to_string());
        Ok(self.solve(&task)?.output)
    }

    pub(crate) fn analyze_code(&mut self, code: &str) -> Result<String, AiError> {
        let task = Task::new_code_analysis(code.to_string());
        Ok(self.solve(&task)?.output)
    }

    pub(crate) fn refactor_code(
        &mut self,
        code: &str,
        instruction: &str,
    ) -> Result<String, AiError> {
        let task = Task::new_refactoring(code.to_string(), instruction.to_string());
        Ok(self.solve(&task)?.output)
    }

    pub(crate) fn evaluate_model(
        &mut self,
        evaluate_data: impl IntoIterator<Item = String>,
    ) -> Result<f64, AiError> {
        self.feedback.evaluate_model(evaluate_data)
    }
    #[allow(dead_code)]
    pub(crate) fn solve_and_return_trace(
        &mut self,
        task: &Task,
    ) -> Result<(TaskResult, SolveTrace), AiError> {
        let res = self.solve(task)?;
        let trace = res.trace().clone();
        Ok((res, trace))
    }
}
