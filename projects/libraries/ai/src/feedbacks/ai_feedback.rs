// projects/libraries/ai/src/ai_feedback.rs
use neural::NeuralSolver;
use neural::feedback::FeedbackType;
use symbolic::{feedback_symbolic::SymbolicFeedback, symbolic_solver::SymbolicSolver};
use tracing::{debug, error, info, warn};

use crate::ai_error::AiError;
use crate::feedbacks::internal::internal_feedback_input::InternalFeedbackInput;
use crate::feedbacks::internal::internal_feedback_meta::InternalFeedbackMeta;
use crate::feedbacks::internal::internal_feedback_verdict::InternalFeedbackVerdict;
use crate::solve_trace::SolveTrace;
use crate::solve_winner::SolveWinner;
use crate::solver_strategy::SolverStrategy;

/// `AiFeedback` is responsible for coordinating the processing of feedback
/// between the symbolic and neural solvers. It acts as the bridge between
/// the feedback domain (`InternalFeedbackInput`) and the neurosymbolic systems.
///
/// # Responsibilities:
/// - Manage the symbolic solver (`SymbolicSolver`) and optionally the neural solver (`NeuralSolver`).
/// - Adjust the solvers based on feedback received via `InternalFeedbackInput`.
/// - Ensure proper handling of feedback types (e.g., Correct, Incorrect, Partial, Rejected).
///
/// # Dependencies:
/// - `SymbolicSolver`: Handles symbolic rule adjustments.
/// - `NeuralSolver`: Handles neural model training and adjustments.
/// - `InternalFeedbackInput`: Provides structured feedback data to process.
pub(crate) struct AiFeedback {
    symbolic: SymbolicSolver,
    neural: Option<NeuralSolver>,
}

impl AiFeedback {
    /// Creates a new instance of `AiFeedback` with an initialized symbolic solver.
    /// The neural solver is not loaded by default and must be initialized separately.
    pub(crate) fn new() -> Result<Self, AiError> {
        Ok(Self {
            symbolic: SymbolicSolver::new()?,
            neural: None,
        })
    }

    /// Loads the neural model into the `AiFeedback` instance.
    ///
    /// # Parameters:
    /// - `model_path`: Path to the neural model file.
    /// - `tokenizer_path`: Path to the tokenizer file.
    pub(crate) fn load_neural_model(
        &mut self,
        model_path: &std::path::Path,
        tokenizer_path: &std::path::Path,
    ) -> Result<(), AiError> {
        self.neural = Some(NeuralSolver::load(model_path, tokenizer_path)?);
        Ok(())
    }

    /// Provides immutable access to the symbolic solver.
    pub(crate) fn symbolic(&self) -> &SymbolicSolver {
        &self.symbolic
    }

    /// Provides mutable access to the symbolic solver.
    pub(crate) fn symbolic_mut(&mut self) -> &mut SymbolicSolver {
        &mut self.symbolic
    }
    #[allow(dead_code)]
    /// Provides immutable access to the neural solver, if it is loaded.
    pub(crate) fn neural(&self) -> Option<&NeuralSolver> {
        self.neural.as_ref()
    }

    /// Provides mutable access to the neural solver, if it is loaded.
    pub(crate) fn neural_mut(&mut self) -> Result<&mut NeuralSolver, AiError> {
        self.neural
            .as_mut()
            .ok_or_else(|| AiError::TaskError("Neural model not loaded".into()))
    }

    /// Saves the neural model from the `AiFeedback` instance.
    ///
    /// # Parameters:
    /// - `model_path`: Path to save the neural model file.
    /// - `tokenizer_path`: Path to save the tokenizer file.
    pub(crate) fn save_neural_model(
        &self,
        model_path: &std::path::Path,
        tokenizer_path: &std::path::Path,
    ) -> Result<(), AiError> {
        if let Some(neural) = &self.neural {
            neural
                .save_model(model_path, Some(tokenizer_path))
                .map_err(|e| AiError::TaskError(format!("Neural save error: {:?}", e)))?;
        } else {
            return Err(AiError::TaskError("Neural model not loaded".into()));
        }

        Ok(())
    }

    /// Adjusts the symbolic and neural solvers based on the provided feedback.
    ///
    /// # Parameters:
    /// - `feedback`: The structured feedback data to process.
    ///
    /// # Behavior:
    /// - If the feedback is rejected or invalid, no adjustments are made.
    /// - If both solvers are active, symbolic and neural adjustments are applied.
    /// - If only one solver is active, adjustments are limited to the available solver.
    ///
    /// # Note:
    /// This is the central method for handling feedback in the system. It ensures
    /// that adjustments are made dynamically based on the available capabilities
    /// (symbolic, neural, or both).
    pub(crate) fn adjust(&mut self, feedback: &InternalFeedbackInput<'_>) -> Result<(), AiError> {
        if matches!(
            feedback.verdict,
            InternalFeedbackVerdict::Rejected | InternalFeedbackVerdict::NoFeedback
        ) {
            info!("Feedback not evaluated; skipping adjustments");
            return Ok(());
        }

        if self.has_symbolic() {
            self.adjust_symbolic(
                feedback.task_input.as_ref(),
                &feedback.verdict,
                &feedback.meta,
            )?;
        }

        if self.has_neural() {
            self.adjust_neural(
                feedback.input.as_ref(),
                feedback.generated_output.as_ref(),
                &feedback.verdict,
                &feedback.meta,
            )?;
        }

        Ok(())
    }

    /// Adjusts the solvers based on combined symbolic and neural feedback.
    ///
    /// # Parameters:
    /// - `symbolic_feedback`: Feedback for the symbolic solver.
    /// - `neural_feedback`: Feedback for the neural solver.
    #[allow(dead_code)]
    pub(crate) fn adjust_combined(
        &mut self,
        _symbolic_feedback: SymbolicFeedback,
        _neural_feedback: FeedbackType,
    ) -> Result<(), AiError> {
        Err(AiError::TaskError(
            "adjust_combined is not implemented (placeholders)".into(),
        ))
    }

    /// Generates a unique hash for feedback based on the input and generated output.
    ///
    /// # Parameters:
    /// - `input`: The input associated with the feedback.
    /// - `generated_output`: The output generated by the system.
    ///
    /// # Returns:
    /// - A unique hexadecimal hash as a string.
    #[inline]
    fn feedback_hash(input: &str, generated_output: &str) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"ai_feedback_v1\0");
        hasher.update(&(input.len() as u64).to_le_bytes());
        hasher.update(input.as_bytes());
        hasher.update(&(generated_output.len() as u64).to_le_bytes());
        hasher.update(generated_output.as_bytes());
        hasher.finalize().to_hex().to_string()
    }

    /// Adjusts the symbolic solver based on the feedback.
    ///
    /// # Parameters:
    /// - `task_input`: The input task associated with the feedback.
    /// - `verdict`: The verdict of the feedback (e.g., Correct, Incorrect).
    /// - `meta`: Metadata associated with the feedback.
    fn adjust_symbolic(
        &mut self,
        task_input: &str,
        verdict: &InternalFeedbackVerdict,
        meta: &InternalFeedbackMeta,
    ) -> Result<(), AiError> {
        let symbolic_feedback = SymbolicFeedback {
            is_positive: verdict.is_positive(),
            metadata: Some(format!(
                "verdict={} confidence={:?} rationale={:?} source={:?}",
                verdict.stable_kind(),
                meta.confidence,
                meta.rationale,
                meta.source
            )),
        };

        self.symbolic.adjust_rules(task_input, symbolic_feedback)?;
        Ok(())
    }

    /// Adjusts the neural solver based on the feedback.
    ///
    /// # Parameters:
    /// - `input`: The input data associated with the feedback.
    /// - `generated_output`: The output generated by the system.
    /// - `verdict`: The verdict of the feedback (e.g., Correct, Incorrect).
    /// - `meta`: Metadata associated with the feedback.
    fn adjust_neural(
        &mut self,
        input: &str,
        generated_output: &str,
        verdict: &InternalFeedbackVerdict,
        _meta: &InternalFeedbackMeta,
    ) -> Result<(), AiError> {
        let Some(neural) = self.neural.as_mut() else {
            warn!("Neural model not loaded, skipping neural feedback");
            return Ok(());
        };

        let feedback_type: FeedbackType = verdict.clone().into();
        let feedback_hash_hex = Self::feedback_hash(input, generated_output);
        let short_hash = &feedback_hash_hex[..feedback_hash_hex.len().min(8)];

        debug!(hash = %short_hash, "Generated feedback hash");

        if neural.record_feedback_if_new(
            &feedback_hash_hex,
            input,
            generated_output,
            feedback_type,
        )? {
            debug!("New feedback recorded");
        } else {
            debug!("Duplicate feedback detected, skipping");
            return Ok(());
        }

        let min_feedback = neural.min_feedback_for_adjustment();
        if min_feedback == 0 {
            error!("Neural solver misconfigured: min_feedback_for_adjustment() == 0");
            return Err(AiError::TaskError(
                "Neural solver misconfigured: min_feedback_for_adjustment() == 0".into(),
            ));
        }

        if neural.pending_since_last_adjust() >= min_feedback {
            info!("Threshold reached, adjusting neural model");
            neural.adjust_model()?;
        }

        debug!(
            kind = ?verdict.stable_kind(),
            input_len = input.len(),
            output_len = generated_output.len(),
            "Recorded neural feedback"
        );

        Ok(())
    }

    /// Adjusts the solvers based on the provided feedback and solve trace.
    ///
    /// # Parameters:
    /// - `feedback`: The structured feedback data to process.
    /// - `trace`: The solve trace providing additional context for adjustment.
    pub(crate) fn adjust_with_trace(
        &mut self,
        feedback: &InternalFeedbackInput<'_>,
        trace: &SolveTrace,
    ) -> Result<(), AiError> {
        if matches!(
            feedback.verdict,
            InternalFeedbackVerdict::Rejected | InternalFeedbackVerdict::NoFeedback
        ) {
            info!("Feedback not evaluated; skipping adjustments");
            return Ok(());
        }

        match trace.strategy {
            SolverStrategy::SymbolicOnly => {
                self.adjust_symbolic(
                    feedback.task_input.as_ref(),
                    &feedback.verdict,
                    &feedback.meta,
                )?;
            }
            SolverStrategy::NeuralOnly => {
                self.adjust_neural(
                    feedback.input.as_ref(),
                    feedback.generated_output.as_ref(),
                    &feedback.verdict,
                    &feedback.meta,
                )?;
            }
            SolverStrategy::SymbolicThenNeural => {
                self.adjust_symbolic(
                    feedback.task_input.as_ref(),
                    &feedback.verdict,
                    &feedback.meta,
                )?;

                if trace.fallback_used || trace.winner == Some(SolveWinner::Neural) {
                    self.adjust_neural(
                        feedback.input.as_ref(),
                        feedback.generated_output.as_ref(),
                        &feedback.verdict,
                        &feedback.meta,
                    )?;
                }
            }
            SolverStrategy::NeuralWithSymbolicValidation => {
                self.adjust_symbolic(
                    feedback.task_input.as_ref(),
                    &feedback.verdict,
                    &feedback.meta,
                )?;
                self.adjust_neural(
                    feedback.input.as_ref(),
                    feedback.generated_output.as_ref(),
                    &feedback.verdict,
                    &feedback.meta,
                )?;
            }
            SolverStrategy::Hybrid => {
                self.adjust_symbolic(
                    feedback.task_input.as_ref(),
                    &feedback.verdict,
                    &feedback.meta,
                )?;

                if trace.winner == Some(SolveWinner::Neural) || trace.neural_ran {
                    self.adjust_neural(
                        feedback.input.as_ref(),
                        feedback.generated_output.as_ref(),
                        &feedback.verdict,
                        &feedback.meta,
                    )?;
                }
            }
        }

        Ok(())
    }

    /// Evaluates the neural model with the provided data.
    ///
    /// # Parameters:
    /// - `evaluate_data`: An iterable containing the data to evaluate.
    ///
    /// # Returns:
    /// - A result containing the evaluation metric (e.g., loss) or an error.
    pub(crate) fn evaluate_model(
        &mut self,
        evaluate_data: impl IntoIterator<Item = String>,
    ) -> Result<f64, AiError> {
        if let Some(neural) = &mut self.neural {
            neural
                .evaluate_model(evaluate_data.into_iter().collect())
                .map_err(|e| {
                    warn!("Neural evaluation failed: {:?}", e);
                    AiError::TaskError(format!("Neural evaluation failed: {:?}", e))
                })
        } else {
            warn!("Neural model not loaded. Cannot evaluate.");
            Err(AiError::TaskError("Neural model not loaded".into()))
        }
    }

    /// Checks if the symbolic solver is available.
    pub(crate) fn has_symbolic(&self) -> bool {
        true // Always available unless changed to Option in the future
    }

    /// Checks if the neural solver is loaded.
    pub(crate) fn has_neural(&self) -> bool {
        self.neural.is_some()
    }
}
