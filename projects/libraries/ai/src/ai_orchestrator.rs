// projects/libraries/ai/src/ai_orchestrator.rs
use std::path;
use tracing::{info, warn};

use crate::{
    ai_error::AiError,
    dispatcher::Dispatcher,
    feedbacks::{InternalFeedbackEvent, ai_feedback::AiFeedback},
    task::Task,
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
        model_path: &path::Path,
        tokenizer_path: &path::Path,
    ) -> Result<(), AiError> {
        info!("Loading neural model...");
        self.feedback
            .load_neural_model(model_path, tokenizer_path)?;
        info!("Neural model loaded successfully");
        Ok(())
    }

    pub(crate) fn save_neural_model(
        &self,
        model_path: &path::Path,
        tokenizer_path: &path::Path,
    ) -> Result<(), AiError> {
        if let Some(neural) = &self.feedback.neural {
            neural.save_model(model_path, Some(tokenizer_path))?;
            info!(
                "Neural model saved successfully to {:?} and {:?}",
                model_path, tokenizer_path
            );
            Ok(())
        } else {
            warn!("Neural model not loaded. Cannot save.");
            Err(AiError::TaskError("Neural model not loaded".into()))
        }
    }

    // --- Feedback API ---

    // The train_with_verdict and train_with_feedback_neural methods have been moved to training.rs.

    /// Adjust feedback for both symbolic and neural solvers.
    /// This is a convenience wrapper around AiFeedback::adjust.
    /// Only accepts primitives.
    pub(crate) fn adjust(&mut self, event: &InternalFeedbackEvent<'_>) -> Result<(), AiError> {
        self.feedback.adjust(event)
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

    pub(crate) fn evaluate_model(&mut self, test_data: Vec<String>) -> Result<f64, AiError> {
        if let Some(neural) = &mut self.feedback.neural {
            neural.evaluate_model(test_data).map_err(|e| {
                warn!("Neural evaluation failed: {:?}", e);
                AiError::TaskError("Neural evaluation failed".into())
            })
        } else {
            warn!("Neural model not loaded. Cannot evaluate.");
            Err(AiError::TaskError("Neural model not loaded".into()))
        }
    }
}
