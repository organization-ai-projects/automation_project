// neural/src/training/feedback.rs
use crate::network::neural_net::{NetworkError, NeuralNetwork};
use common_time::timestamp_utils::format_timestamp;
use ndarray::Array1;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FeedbackError {
    #[error("Invalid feedback format: {0}")]
    InvalidFormat(String),
    #[error("Network error: {0}")]
    NetworkError(#[from] NetworkError),
    #[error("Training error: {0}")]
    TrainingError(String),
}

/// Type de feedback que l'utilisateur peut donner
#[derive(Debug, Clone)]
pub enum FeedbackType {
    /// Code généré était correct
    Correct,
    /// Code généré était incorrect, voici la bonne version
    Incorrect { expected_output: String },
    /// Code était partiellement correct, ajustement nécessaire
    Partial { correction: String, confidence: f32 },
}

/// Structure du feedback utilisateur
#[derive(Debug, Clone)]
pub struct UserFeedback {
    /// Input original qui a produit la génération
    pub input: String,
    /// Output généré par le modèle
    pub generated_output: String,
    /// Type de feedback
    pub feedback_type: FeedbackType,
    /// Timestamp
    pub timestamp: std::time::SystemTime,
}

impl UserFeedback {
    pub fn formatted_timestamp(&self) -> String {
        format_timestamp(
            self.timestamp
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        )
    }
}

/// Gestionnaire d'ajustement du modèle via feedback
pub struct FeedbackAdjuster {
    /// Historique des feedbacks
    feedback_history: Vec<UserFeedback>,
    /// Configuration d'ajustement
    config: FeedbackConfig,
}

#[derive(Debug, Clone)]
pub struct FeedbackConfig {
    /// Learning rate pour les ajustements
    pub learning_rate: f64,
    /// Nombre minimum de feedbacks avant ajustement
    pub min_feedback_count: usize,
    /// Batch size pour les ajustements
    pub batch_size: usize,
    /// Sauvegarder l'historique sur disque
    pub save_history: bool,
    pub history_path: std::path::PathBuf,
}

impl Default for FeedbackConfig {
    fn default() -> Self {
        Self {
            learning_rate: 0.001, // Plus petit que training initial
            min_feedback_count: 10,
            batch_size: 5,
            save_history: true,
            history_path: "data/feedback_history.json".into(),
        }
    }
}

impl FeedbackAdjuster {
    pub fn new(config: FeedbackConfig) -> Self {
        let feedback_history = if config.save_history && config.history_path.exists() {
            Self::load_history(&config.history_path).unwrap_or_default()
        } else {
            Vec::new()
        };

        Self {
            feedback_history,
            config,
        }
    }

    /// Enregistre un feedback utilisateur
    pub fn record_feedback(&mut self, feedback: UserFeedback) -> Result<(), FeedbackError> {
        println!(
            "Recording feedback: {:?} for input: '{}'",
            feedback.feedback_type,
            feedback.input.chars().take(50).collect::<String>()
        );

        self.feedback_history.push(feedback);

        // Sauvegarder sur disque si configuré
        if self.config.save_history {
            self.save_history()?;
        }

        Ok(())
    }

    /// Ajuste le modèle basé sur les feedbacks accumulés
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

        println!(
            "Adjusting model with {} feedback examples",
            self.feedback_history.len()
        );

        let mut metrics = AdjustmentMetrics::new();

        // Filtrer les feedbacks négatifs (où correction est nécessaire)
        let training_examples: Vec<_> = self
            .feedback_history
            .iter()
            .filter_map(|fb| match &fb.feedback_type {
                FeedbackType::Incorrect { expected_output } => {
                    Some((fb.input.clone(), expected_output.clone()))
                }
                FeedbackType::Partial { correction, .. } => {
                    Some((fb.input.clone(), correction.clone()))
                }
                FeedbackType::Correct => None, // Pas besoin d'ajuster si correct
            })
            .collect();

        if training_examples.is_empty() {
            println!("No negative feedback to learn from");
            return Ok(metrics);
        }

        println!("Training on {} negative examples", training_examples.len());

        // Fine-tuning en mini-batches
        for (batch_idx, batch) in training_examples.chunks(self.config.batch_size).enumerate() {
            let mut batch_loss = 0.0;

            for (input, expected) in batch {
                // Tokenize input et expected output
                let input_tokens = tokenizer.encode(input);
                let expected_tokens = tokenizer.encode(expected);

                // Convert to ndarray
                let input_vec = self.tokens_to_vector(&input_tokens, tokenizer.vocab_size());
                let target_vec = self.tokens_to_vector(&expected_tokens, tokenizer.vocab_size());

                // Forward pass
                let output = model.forward(&input_vec)?;

                // Calculer l'erreur
                let error = &target_vec - &output;
                let loss = error.mapv(|x| x * x).sum() / error.len() as f64;
                batch_loss += loss;

                // Backward pass avec learning rate réduit
                // Note: Pour un vrai fine-tuning, il faudrait backprop à travers toutes les couches
                // Ici on fait un ajustement simple sur la dernière couche
                for layer in model.layers_mut().iter_mut().rev().take(1) {
                    layer.backward(&error, self.config.learning_rate)?;
                }
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

    /// Convertit des tokens en vecteur one-hot moyenné
    fn tokens_to_vector(&self, tokens: &[usize], vocab_size: usize) -> Array1<f64> {
        let mut vec = Array1::<f64>::zeros(vocab_size);

        for &token in tokens {
            if token < vocab_size {
                vec[token] += 1.0;
            }
        }

        // Normaliser
        if !tokens.is_empty() {
            vec / tokens.len() as f64
        } else {
            vec
        }
    }

    /// Statistiques sur les feedbacks
    pub fn feedback_stats(&self) -> FeedbackStats {
        let total = self.feedback_history.len();
        let mut correct = 0;
        let mut incorrect = 0;
        let mut partial = 0;

        for fb in &self.feedback_history {
            match fb.feedback_type {
                FeedbackType::Correct => correct += 1,
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

    /// Réinitialiser l'historique de feedback
    pub fn clear_history(&mut self) -> Result<(), FeedbackError> {
        self.feedback_history.clear();
        if self.config.save_history {
            self.save_history()?;
        }
        Ok(())
    }

    fn save_history(&self) -> Result<(), FeedbackError> {
        let json = serde_json::to_string_pretty(&self.feedback_history)
            .map_err(|e| FeedbackError::TrainingError(e.to_string()))?;
        std::fs::write(&self.config.history_path, json)
            .map_err(|e| FeedbackError::TrainingError(e.to_string()))?;
        Ok(())
    }

    fn load_history(path: &std::path::Path) -> Result<Vec<UserFeedback>, FeedbackError> {
        let json = std::fs::read_to_string(path)
            .map_err(|e| FeedbackError::TrainingError(e.to_string()))?;
        let history =
            serde_json::from_str(&json).map_err(|e| FeedbackError::TrainingError(e.to_string()))?;
        Ok(history)
    }
}

#[derive(Debug, Clone)]
pub struct AdjustmentMetrics {
    pub total_examples: usize,
    pub batch_losses: Vec<f64>,
    pub avg_loss: f64,
}

impl AdjustmentMetrics {
    fn new() -> Self {
        Self {
            total_examples: 0,
            batch_losses: Vec::new(),
            avg_loss: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FeedbackStats {
    pub total: usize,
    pub correct: usize,
    pub incorrect: usize,
    pub partial: usize,
    pub accuracy: f64,
}

// Implémentation de Serialize/Deserialize pour UserFeedback
use serde::{Deserialize, Serialize};

impl Serialize for UserFeedback {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("UserFeedback", 4)?;
        state.serialize_field("input", &self.input)?;
        state.serialize_field("generated_output", &self.generated_output)?;
        state.serialize_field("feedback_type", &format!("{:?}", self.feedback_type))?;
        state.serialize_field(
            "timestamp",
            &self
                .timestamp
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        )?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for UserFeedback {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Implémentation simplifiée
        todo!("Implement deserialize for UserFeedback")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feedback_recording() {
        let mut adjuster = FeedbackAdjuster::new(FeedbackConfig {
            save_history: false,
            ..Default::default()
        });

        let feedback = UserFeedback {
            input: "create function".to_string(),
            generated_output: "fn bad() {}".to_string(),
            feedback_type: FeedbackType::Incorrect {
                expected_output: "fn good() {}".to_string(),
            },
            timestamp: std::time::SystemTime::now(),
        };

        adjuster.record_feedback(feedback).unwrap();

        let stats = adjuster.feedback_stats();
        assert_eq!(stats.total, 1);
        assert_eq!(stats.incorrect, 1);
    }
}
