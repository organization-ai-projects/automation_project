// projects/libraries/ai/src/ai_feedback.rs
use neural::NeuralSolver;
use neural::feedback::feedback_type::FeedbackMetadata;
use neural::feedback::{FeedbackType, UserFeedback};
use symbolic::{feedback_symbolic::SymbolicFeedback, symbolic_solver::SymbolicSolver};
use tracing::{info, warn};

use crate::ai_error::AiError;
use crate::feedbacks::{InternalFeedbackEvent, InternalFeedbackMeta, InternalFeedbackVerdict};

pub(crate) struct AiFeedback {
    pub(crate) symbolic: SymbolicSolver,
    pub(crate) neural: Option<NeuralSolver>,
}

impl AiFeedback {
    pub(crate) fn new() -> Result<Self, AiError> {
        Ok(Self {
            symbolic: SymbolicSolver::new()?,
            neural: None,
        })
    }

    pub(crate) fn load_neural_model(
        &mut self,
        model_path: &std::path::Path,
        tokenizer_path: &std::path::Path,
    ) -> Result<(), AiError> {
        self.neural = Some(NeuralSolver::load(model_path, tokenizer_path)?);
        Ok(())
    }

    pub(crate) fn neural_mut(&mut self) -> Result<&mut NeuralSolver, AiError> {
        self.neural
            .as_mut()
            .ok_or_else(|| AiError::TaskError("Neural model not loaded".into()))
    }

    pub(crate) fn adjust(&mut self, event: &InternalFeedbackEvent<'_>) -> Result<(), AiError> {
        self.adjust_symbolic(event.task_input.as_ref(), &event.verdict, &event.meta)?;
        self.adjust_neural(
            event.input.as_ref(),
            event.generated_output.as_ref(),
            &event.verdict,
            &event.meta,
        )?;
        Ok(())
    }

    fn adjust_symbolic(
        &mut self,
        task_input: &str,
        verdict: &InternalFeedbackVerdict,
        meta: &InternalFeedbackMeta,
    ) -> Result<(), AiError> {
        let symbolic_feedback = SymbolicFeedback {
            is_positive: matches!(verdict, InternalFeedbackVerdict::Correct),
            metadata: Some(format!(
                "kind={:?} source={:?} rationale={:?}",
                verdict, meta.source, meta.rationale
            )),
        };

        self.symbolic.adjust_rules(task_input, symbolic_feedback)?;
        Ok(())
    }

    fn adjust_neural(
        &mut self,
        input: &str,
        generated_output: &str,
        verdict: &InternalFeedbackVerdict,
        meta: &InternalFeedbackMeta,
    ) -> Result<(), AiError> {
        let Some(neural) = self.neural.as_mut() else {
            warn!("Neural model not loaded, skipping neural feedback");
            return Ok(());
        };

        if matches!(verdict, InternalFeedbackVerdict::Rejected) {
            info!("Rejected verdict: skipping neural training");
            return Ok(());
        }

        // Dedup stable: blake3 over structured bytes (no giant format!)
        // Requires dependency: blake3 = "1"
        let tag = verdict.stable_kind();
        let payload = verdict.stable_payload();

        let mut hasher = blake3::Hasher::new();
        hasher.update(&(input.len() as u64).to_le_bytes());
        hasher.update(input.as_bytes());
        hasher.update(&(generated_output.len() as u64).to_le_bytes());
        hasher.update(generated_output.as_bytes());
        hasher.update(tag.as_bytes());
        if let Some(p) = payload {
            hasher.update(&(p.len() as u64).to_le_bytes());
            hasher.update(p.as_bytes());
        }
        let feedback_hash_hex = hasher.finalize().to_hex().to_string();

        if neural.has_seen_feedback(&feedback_hash_hex) {
            info!("Duplicate feedback detected, skipping");
            return Ok(());
        }

        let metadata = FeedbackMetadata {
            confidence: meta.confidence,
            rationale: meta.rationale.clone(),
            source: meta.source.clone(),
        };

        let feedback_type = match verdict {
            InternalFeedbackVerdict::Correct => FeedbackType::Correct { metadata },
            InternalFeedbackVerdict::Incorrect { expected_output } => FeedbackType::Incorrect {
                expected_output: expected_output.clone(),
                metadata,
            },
            InternalFeedbackVerdict::Partial { correction } => FeedbackType::Partial {
                correction: correction.clone(),
                metadata,
            },
            InternalFeedbackVerdict::Rejected => return Ok(()),
        };

        let user_feedback = UserFeedback::from_parts(input, generated_output, feedback_type);
        neural.record_feedback(&user_feedback)?;

        let min_feedback = neural.min_feedback_for_adjustment();
        if min_feedback == 0 {
            return Err(AiError::TaskError(
                "Neural solver misconfigured: min_feedback_for_adjustment() == 0".into(),
            ));
        }

        if neural.feedback_count() % min_feedback == 0 {
            info!("Threshold reached, adjusting neural model");
            neural.adjust_model()?;
        }

        info!(
            kind = ?verdict,
            input_len = input.len(),
            output_len = generated_output.len(),
            "Recorded neural feedback"
        );

        Ok(())
    }
}
