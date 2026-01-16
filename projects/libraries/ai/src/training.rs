use crate::{ai_error::AiError, ai_orchestrator::AiOrchestrator, task::Task};
use common_json::{from_json_str, to_string};
use neural::feedback::{FeedbackType, UserFeedback, feedback_type::FeedbackMetadata};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use symbolic::feedback_symbolic::SymbolicFeedback;
use tracing::{info, warn};

impl AiOrchestrator {
    pub(crate) fn train_with_verdict(
        &mut self,
        task: &Task,
        input: &str,
        generated_output: &str,
        ok: bool,
    ) -> Result<(), AiError> {
        let feedback = UserFeedback::new(
            input,
            generated_output,
            if ok {
                FeedbackType::Correct {
                    metadata: FeedbackMetadata {
                        confidence: Some(1.0),
                        rationale: Some("Feedback positif".to_string()),
                        source: Some("user".to_string()),
                    },
                }
            } else {
                FeedbackType::Incorrect {
                    expected_output: generated_output.to_string(),
                    metadata: FeedbackMetadata {
                        confidence: Some(0.0),
                        rationale: Some("Feedback négatif".to_string()),
                        source: Some("user".to_string()),
                    },
                }
            },
        );
        self.train_with_feedback_neural(task, &feedback)
    }

    pub(crate) fn train_with_feedback_neural(
        &mut self,
        task: &Task,
        feedback: &UserFeedback,
    ) -> Result<(), AiError> {
        info!(
            "Training with neural feedback for task: {:?}",
            task.task_type()
        );

        if let Some(neural) = &mut self.feedback.neural {
            neural.record_feedback(feedback)?;
            if let Err(e) = neural.save_model(
                "models/neural_model.bin".as_ref(),
                Some("models/tokenizer.bin".as_ref()),
            ) {
                warn!("Failed to save neural model: {:?}", e);
            }
        } else {
            warn!("Neural model not loaded, skipping neural feedback");
        }

        let symbolic_feedback = SymbolicFeedback {
            is_positive: matches!(&feedback.feedback_type, FeedbackType::Correct { .. }),
            metadata: Some(format!(
                "Input: {}, Output: {}",
                feedback.input, feedback.generated_output
            )),
        };
        self.feedback
            .symbolic
            .adjust_rules(task.input(), symbolic_feedback)?;

        Ok(())
    }

    pub(crate) fn train_neural(
        &mut self,
        training_data: Vec<String>,
        model_path: &Path,
    ) -> Result<(), AiError> {
        self.feedback
            .neural_mut()?
            .train(training_data, model_path)?;
        Ok(())
    }

    pub(crate) fn append_training_example(
        &self,
        replay_path: &std::path::Path,
        example: &UserFeedback,
    ) -> Result<(), AiError> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(replay_path)
            .map_err(|e| AiError::TaskError(e.to_string()))?;
        let serialized = to_string(example)?;
        writeln!(file, "{}", serialized).map_err(|e| AiError::TaskError(e.to_string()))?;
        Ok(())
    }

    pub(crate) fn append_training_example_json(
        &self,
        replay_path: &std::path::Path,
        example_json: &str,
    ) -> Result<(), AiError> {
        let mut example: UserFeedback = from_json_str(example_json)?;

        if example.timestamp_unix_secs == 0 {
            example.timestamp_unix_secs = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
        }

        self.append_training_example(replay_path, &example)
    }

    pub(crate) fn load_training_examples_as_strings(
        &self,
        replay_path: &std::path::Path,
    ) -> Result<Vec<String>, AiError> {
        if !replay_path.exists() {
            return Ok(vec![]);
        }
        let file = OpenOptions::new()
            .read(true)
            .open(replay_path)
            .map_err(|e| AiError::TaskError(e.to_string()))?;
        let reader = BufReader::new(file);

        let mut out = Vec::new();
        for line in reader.lines() {
            let mut feedback: UserFeedback =
                from_json_str(&line.map_err(|e| AiError::TaskError(e.to_string()))?)
                    .map_err(|e| AiError::TaskError(e.to_string()))?;

            if feedback.timestamp_unix_secs == 0 {
                feedback.timestamp_unix_secs = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
            }

            out.push(to_string(&feedback).map_err(|e| AiError::TaskError(e.to_string()))?);
        }
        Ok(out)
    }
}
