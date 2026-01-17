// projects/libraries/neural/src/feedback/feedback_adjuster.rs
use common_json::json;
use ndarray::Array1;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::{
    feedback::{
        AdjustmentMetrics, FeedbackConfig, FeedbackError, FeedbackStats, FeedbackType, UserFeedback,
    },
    network::neural_net::NeuralNetwork,
};

/// Model adjustment manager via feedback
pub struct FeedbackAdjuster {
    /// Feedback history
    feedback_history: Vec<UserFeedback>,
    /// Adjustment configuration
    config: FeedbackConfig,
    /// Set of seen feedbacks (to avoid duplicates)
    seen_feedback: std::collections::HashSet<u64>,
}

impl FeedbackAdjuster {
    /// Creates a new FeedbackAdjuster instance
    pub fn new(config: FeedbackConfig) -> Self {
        let feedback_history = if config.save_history && config.history_path.exists() {
            Self::load_history(&config.history_path).unwrap_or_default()
        } else {
            Vec::new()
        };

        Self {
            feedback_history,
            config,
            seen_feedback: std::collections::HashSet::new(),
        }
    }

    /// Records user feedback
    pub fn record_feedback(&mut self, feedback: &UserFeedback) -> Result<(), FeedbackError> {
        // Adds a small percentage of Correct feedbacks for stabilization
        match &feedback.feedback_type {
            FeedbackType::Incorrect { .. } => {
                self.feedback_history.push(feedback.clone());
            }
            FeedbackType::Partial { .. } => {
                self.feedback_history.push(feedback.clone());
            }
            FeedbackType::Correct { metadata: _ } => {
                if rand::random::<f32>() < self.config.correct_sampling_ratio {
                    self.feedback_history.push(feedback.clone());
                }
            }
        };

        // Mark the feedback as "seen"
        let mut hasher = DefaultHasher::new();
        feedback.hash(&mut hasher);
        let feedback_hash = hasher.finish();
        self.seen_feedback.insert(feedback_hash);

        // Replace the log to avoid leaking sensitive data
        let preview_len = feedback.input.len();
        println!(
            "Recording feedback: {:?} (input_len={})",
            feedback.feedback_type, preview_len
        );

        // Save to disk if configured
        if self.config.save_history {
            self.save_history()?;
        }

        Ok(())
    }

    /// Adjusts the model based on accumulated feedback
    pub fn adjust_model(
        &mut self,
        model: &mut NeuralNetwork,
        tokenizer: &crate::tokenization::rust_tokenizer::RustTokenizer,
    ) -> Result<AdjustmentMetrics, FeedbackError> {
        if self.feedback_history.len() < self.config.min_feedback_count {
            return Err(FeedbackError::TrainingError(format!(
                "Not enough feedback: {} < {}",
                self.feedback_history.len(),
                self.config.min_feedback_count
            )));
        }

        // Check model and tokenizer dimensions
        if model.input_size() != tokenizer.vocab_size() {
            return Err(FeedbackError::TrainingError(format!(
                "Model input size ({}) does not match tokenizer vocab size ({})",
                model.input_size(),
                tokenizer.vocab_size()
            )));
        }

        if model.output_size() != tokenizer.vocab_size() {
            return Err(FeedbackError::TrainingError(format!(
                "Model output size ({}) does not match tokenizer vocab size ({})",
                model.output_size(),
                tokenizer.vocab_size()
            )));
        }

        println!(
            "Adjusting model with {} feedback examples",
            self.feedback_history.len()
        );

        let mut metrics = AdjustmentMetrics::new();

        // Filter negative feedbacks (where correction is needed)
        let training_examples: Vec<(&str, String)> = self
            .feedback_history
            .iter()
            .filter_map(|fb| {
                let target = match &fb.feedback_type {
                    FeedbackType::Incorrect {
                        expected_output, ..
                    } => expected_output.clone(),
                    FeedbackType::Partial { correction, .. } => correction.clone(),
                    FeedbackType::Correct { .. } => return None,
                };
                Some((fb.input.as_str(), target))
            })
            .collect();

        if training_examples.is_empty() {
            println!("No valid feedback to learn from");
            return Ok(metrics);
        }

        println!("Training on {} negative examples", training_examples.len());

        // Check batch size to avoid division by zero
        if self.config.batch_size == 0 {
            return Err(FeedbackError::TrainingError(
                "batch_size must be > 0".into(),
            ));
        }

        // Fine-tuning in mini-batches
        for (batch_idx, batch) in training_examples.chunks(self.config.batch_size).enumerate() {
            let mut batch_loss = 0.0;

            for (input, expected) in batch {
                // Tokenize input and expected output
                let input_tokens = tokenizer.encode(input);
                let expected_tokens = tokenizer.encode(expected);

                // Convert to ndarray
                let input_vec = self.tokens_to_vector(&input_tokens, tokenizer.vocab_size());
                let target_vec = self.tokens_to_vector(&expected_tokens, tokenizer.vocab_size());

                // Forward pass and backward
                model.forward(&input_vec)?;
                let loss = model.backward(&target_vec, self.config.learning_rate)?;
                batch_loss += loss;
            }

            let avg_batch_loss = batch_loss / batch.len() as f64;
            metrics.batch_losses.push(avg_batch_loss);

            println!("Batch {}: loss = {:.6}", batch_idx, avg_batch_loss);
        }

        metrics.total_examples = training_examples.len();
        metrics.avg_loss =
            metrics.batch_losses.iter().sum::<f64>() / metrics.batch_losses.len() as f64;

        println!("Adjustment complete: avg_loss = {:.6}", metrics.avg_loss);

        Ok(metrics)
    }

    /// Applies adjustments based on feedback
    pub fn apply_feedback(&mut self) -> Result<(), FeedbackError> {
        println!("Applying feedback adjustments...");
        if self.feedback_history.len() < self.config.min_feedback_count {
            return Err(FeedbackError::InsufficientFeedback);
        }

        // Placeholder implementation: adjust parameters
        for feedback in &self.feedback_history {
            println!("Adjusting based on feedback: {:?}", feedback);
        }
        Ok(())
    }

    /// Converts tokens to averaged one-hot vector
    fn tokens_to_vector(&self, tokens: &[usize], vocab_size: usize) -> Array1<f64> {
        let mut vec = Array1::<f64>::zeros(vocab_size);

        for &token in tokens {
            if token < vocab_size {
                vec[token] += 1.0;
            }
        }

        // Normalize
        if !tokens.is_empty() {
            vec / tokens.len() as f64
        } else {
            vec
        }
    }

    /// Feedback statistics
    pub fn feedback_stats(&self) -> FeedbackStats {
        let total = self.feedback_history.len();
        let mut correct = 0;
        let mut incorrect = 0;
        let mut partial = 0;

        for fb in &self.feedback_history {
            match fb.feedback_type {
                FeedbackType::Correct { metadata: _ } => correct += 1,
                FeedbackType::Incorrect { .. } => incorrect += 1,
                FeedbackType::Partial { .. } => partial += 1,
            }
        }

        FeedbackStats {
            total,
            correct,
            incorrect,
            partial,
            accuracy: if total > 0 {
                correct as f64 / total as f64
            } else {
                0.0
            },
        }
    }

    /// Resets feedback history
    pub fn clear_history(&mut self) -> Result<(), FeedbackError> {
        self.feedback_history.clear();
        if self.config.save_history {
            self.save_history()?;
        }
        Ok(())
    }

    pub fn has_feedback(&self, feedback_hash: &str) -> bool {
        let mut hasher = DefaultHasher::new();
        feedback_hash.hash(&mut hasher);
        let target_hash = hasher.finish();

        self.seen_feedback.contains(&target_hash)
    }

    pub fn feedback_count(&self) -> usize {
        self.feedback_history.len()
    }

    pub fn min_feedback_for_adjustment(&self) -> usize {
        self.config.min_feedback_count
    }

    /// Returns the number of pending feedbacks since the last adjustment
    pub fn pending_feedback_count(&self) -> usize {
        self.feedback_history.len() // Placeholder: adjust according to real logic
    }

    fn save_history(&self) -> Result<(), FeedbackError> {
        let json = json::to_json_string_pretty(&self.feedback_history)
            .map_err(|e| FeedbackError::TrainingError(e.to_string()))?;
        std::fs::write(&self.config.history_path, json)
            .map_err(|e| FeedbackError::TrainingError(e.to_string()))?;
        Ok(())
    }

    fn load_history(path: &std::path::Path) -> Result<Vec<UserFeedback>, FeedbackError> {
        let json = std::fs::read_to_string(path)
            .map_err(|e| FeedbackError::TrainingError(e.to_string()))?;
        let history =
            json::from_json_str(&json).map_err(|e| FeedbackError::TrainingError(e.to_string()))?;
        Ok(history)
    }
}
