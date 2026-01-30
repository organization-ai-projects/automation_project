// projects/libraries/neural/src/network/neural_network.rs
use common_json::json;
use ndarray::Array1;
use serde::{Deserialize, Serialize};

use crate::network::{Layer, LayerConfig, NetworkError};

/// Multi-layer neural network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralNetwork {
    layers: Vec<Layer>,
}

impl NeuralNetwork {
    pub fn new(layer_configs: Vec<LayerConfig>) -> Result<Self, NetworkError> {
        if layer_configs.is_empty() {
            return Err(NetworkError::InvalidConfig(
                "Network must have at least one layer".to_string(),
            ));
        }

        // Validate dimensions
        for i in 1..layer_configs.len() {
            if layer_configs[i].input_size != layer_configs[i - 1].output_size {
                return Err(NetworkError::InvalidConfig(format!(
                    "Layer {} input size ({}) doesn't match previous layer output size ({})",
                    i,
                    layer_configs[i].input_size,
                    layer_configs[i - 1].output_size
                )));
            }
        }

        let layers = layer_configs.into_iter().map(Layer::new).collect();

        Ok(Self { layers })
    }

    pub fn load(path: &std::path::Path) -> Result<Self, NetworkError> {
        // Load a NeuralNetwork model from a file (simplified implementation)
        let data = std::fs::read_to_string(path)
            .map_err(|e| NetworkError::InvalidConfig(e.to_string()))?;
        let layers: Vec<Layer> =
            json::from_json_str(&data).map_err(|e| NetworkError::InvalidConfig(e.to_string()))?;
        Ok(Self { layers })
    }

    pub fn forward(&mut self, input: &Array1<f64>) -> Result<Array1<f64>, NetworkError> {
        let mut current = input.clone();

        for layer in &mut self.layers {
            current = layer.forward(&current)?;
        }

        Ok(current)
    }

    pub fn backward(
        &mut self,
        target: &Array1<f64>,
        learning_rate: f64,
    ) -> Result<f64, NetworkError> {
        // Get output from last layer
        let output = self
            .layers
            .last()
            .and_then(|l| l.last_output.as_ref())
            .ok_or_else(|| {
                NetworkError::InvalidConfig("Must call forward() before backward()".to_string())
            })?;

        // Calculate initial gradient (MSE loss)
        let mut gradient = output - target;
        let n = gradient.len() as f64;
        let loss = gradient.mapv(|x| x * x).sum() / n;
        gradient = (2.0 / n) * gradient;

        // Backpropagate through layers
        for layer in self.layers.iter_mut().rev() {
            gradient = layer.backward(&gradient, learning_rate)?;
        }

        Ok(loss)
    }

    pub fn update_weights(&mut self, tokens: &[usize]) -> Result<(), NetworkError> {
        println!("Updating weights with {} tokens", tokens.len());
        // Placeholder implementation: updating layer weights
        for layer in &mut self.layers {
            layer.update(tokens)?;
        }
        Ok(())
    }

    pub fn input_size(&self) -> usize {
        self.layers.first().map(|l| l.weights.ncols()).unwrap_or(0)
    }

    pub fn output_size(&self) -> usize {
        self.layers.last().map(|l| l.weights.nrows()).unwrap_or(0)
    }

    pub fn layers_mut(&mut self) -> &mut Vec<Layer> {
        &mut self.layers
    }
}

// Alias for compatibility with older code
pub type SimpleNeuralNet = NeuralNetwork;
