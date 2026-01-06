// projects/libraries/neural/src/training/train_model.rs
use crate::network::neural_net::{Activation, LayerConfig, SimpleNeuralNet, WeightInit};
use ndarray::Array1;
use thiserror::Error;
use tracing::{debug, info};

#[derive(Debug, Error)]
pub enum TrainingError {
    #[error("Invalid input data: {0}")]
    InvalidInput(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },
}

#[derive(Debug, Clone)]
pub struct TrainingConfig {
    pub learning_rate: f64,
    pub epochs: usize,
    pub batch_size: usize,
    pub validation_split: f32,
    pub early_stopping_patience: Option<usize>,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            learning_rate: 0.01,
            epochs: 100,
            batch_size: 32,
            validation_split: 0.2,
            early_stopping_patience: Some(10),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TrainingExample {
    pub input: Array1<f64>,
    pub target: f64,
}

#[derive(Debug)]
pub struct TrainingMetrics {
    pub train_losses: Vec<f64>,
    pub val_losses: Vec<f64>,
    pub best_epoch: usize,
    pub final_loss: f64,
}

pub struct Trainer {
    config: TrainingConfig,
    network: SimpleNeuralNet,
}

impl Trainer {
    pub fn new(input_size: usize, output_size: usize, config: TrainingConfig) -> Self {
        let layer_configs = vec![LayerConfig {
            input_size,
            output_size,
            activation: Activation::ReLU,
            weight_init: WeightInit::He,
        }];

        Self {
            config,
            network: SimpleNeuralNet::new(layer_configs).unwrap(), // Gestion explicite des erreurs
        }
    }

    /// Parse raw text data into training examples
    fn parse_data(&self, data: &str) -> Result<Vec<TrainingExample>, TrainingError> {
        data.lines()
            .enumerate()
            .map(|(idx, line)| {
                // Format attendu: "input_text|target_value"
                let parts: Vec<&str> = line.split('|').collect();

                if parts.len() != 2 {
                    return Err(TrainingError::InvalidInput(format!(
                        "Line {}: expected format 'input|target'",
                        idx + 1
                    )));
                }

                let input_text = parts[0];
                let target = parts[1].parse::<f64>().map_err(|e| {
                    TrainingError::InvalidInput(format!(
                        "Line {}: invalid target value: {}",
                        idx + 1,
                        e
                    ))
                })?;

                let input = self.tokenize(input_text)?;

                Ok(TrainingExample { input, target })
            })
            .collect()
    }

    /// Tokenize text into numerical representation
    fn tokenize(&self, text: &str) -> Result<Array1<f64>, TrainingError> {
        let tokens: Vec<f64> = text
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .map(|c| (c as u32 as f64) / 1114111.0) // Normalize by max Unicode
            .collect();

        if tokens.is_empty() {
            return Err(TrainingError::InvalidInput(
                "Empty tokenized input".to_string(),
            ));
        }

        Ok(Array1::from_vec(tokens))
    }

    /// Split data into train and validation sets
    fn split_data(
        &self,
        examples: Vec<TrainingExample>,
    ) -> (Vec<TrainingExample>, Vec<TrainingExample>) {
        let val_size = (examples.len() as f32 * self.config.validation_split) as usize;
        let train_size = examples.len() - val_size;

        let (train, val) = examples.split_at(train_size);
        (train.to_vec(), val.to_vec())
    }

    /// Calculate loss on a dataset
    fn calculate_loss(&mut self, examples: &[TrainingExample]) -> f64 {
        let total_loss: f64 = examples
            .iter()
            .filter_map(|ex| {
                match self.network.forward(&ex.input) {
                    Ok(output) => {
                        let error = ex.target - output[0];
                        Some(error * error) // MSE
                    }
                    Err(e) => {
                        debug!("Error during forward pass: {:?}", e);
                        None
                    }
                }
            })
            .sum();

        total_loss / examples.len() as f64
    }

    /// Train the model
    pub fn train(&mut self, data: &str) -> Result<TrainingMetrics, TrainingError> {
        info!("Starting training");

        // Parse and split data
        let examples = self.parse_data(data)?;
        let (train_examples, val_examples) = self.split_data(examples);

        info!("Training set: {} examples", train_examples.len());
        info!("Validation set: {} examples", val_examples.len());

        let mut metrics = TrainingMetrics {
            train_losses: Vec::with_capacity(self.config.epochs),
            val_losses: Vec::with_capacity(self.config.epochs),
            best_epoch: 0,
            final_loss: f64::MAX,
        };

        let mut best_val_loss = f64::MAX;
        let mut patience_counter = 0;

        // Training loop
        for epoch in 0..self.config.epochs {
            debug!("Epoch {}/{}", epoch + 1, self.config.epochs);

            // Train on all examples
            for example in &train_examples {
                if let Ok(output) = self.network.forward(&example.input) {
                    let error = example.target - output[0];
                    debug!("Error: {:.4}", error);
                    // Placeholder for backpropagation logic
                    // self.network.backpropagate(&example.input, error, self.config.learning_rate);
                } else {
                    debug!("Skipping example due to forward pass error");
                }
            }

            // Calculate losses
            let train_loss = self.calculate_loss(&train_examples);
            let val_loss = self.calculate_loss(&val_examples);

            metrics.train_losses.push(train_loss);
            metrics.val_losses.push(val_loss);

            info!(
                "Epoch {}: train_loss={:.4}, val_loss={:.4}",
                epoch + 1,
                train_loss,
                val_loss
            );

            // Track best model
            if val_loss < best_val_loss {
                best_val_loss = val_loss;
                metrics.best_epoch = epoch;
                patience_counter = 0;
            } else {
                patience_counter += 1;
            }

            // Early stopping
            if self.config.early_stopping_patience.is_some_and(|patience| patience_counter >= patience) {
                info!("Early stopping at epoch {}", epoch + 1);
                break;
            }
        }

        metrics.final_loss = best_val_loss;
        info!(
            "Training complete. Best validation loss: {:.4}",
            best_val_loss
        );

        Ok(metrics)
    }

    pub fn network(&self) -> &SimpleNeuralNet {
        &self.network
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenization() {
        let config = TrainingConfig::default();
        let trainer = Trainer::new(10, 1, config);

        let result = trainer.tokenize("test");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 4);
    }

    #[test]
    fn test_parse_data() {
        let config = TrainingConfig::default();
        let trainer = Trainer::new(10, 1, config);

        let data = "hello|1.0\nworld|0.5";
        let examples = trainer.parse_data(data).unwrap();

        assert_eq!(examples.len(), 2);
        assert_eq!(examples[0].target, 1.0);
        assert_eq!(examples[1].target, 0.5);
    }
}
