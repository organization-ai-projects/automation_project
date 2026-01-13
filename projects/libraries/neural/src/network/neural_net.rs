// projects/libraries/neural/src/network/neural_net.rs
use ndarray::{Array1, Array2, Axis};
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },
    #[error("Invalid layer configuration: {0}")]
    InvalidConfig(String),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Activation {
    ReLU,
    Sigmoid,
    Tanh,
    Linear,
}

impl Activation {
    pub fn apply(&self, x: f64) -> f64 {
        match self {
            Activation::ReLU => x.max(0.0),
            Activation::Sigmoid => 1.0 / (1.0 + (-x).exp()),
            Activation::Tanh => x.tanh(),
            Activation::Linear => x,
        }
    }

    pub fn derivative(&self, x: f64) -> f64 {
        match self {
            Activation::ReLU => {
                if x > 0.0 {
                    1.0
                } else {
                    0.0
                }
            }
            Activation::Sigmoid => {
                let s = self.apply(x);
                s * (1.0 - s)
            }
            Activation::Tanh => 1.0 - x.tanh().powi(2),
            Activation::Linear => 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum WeightInit {
    Xavier,
    He,
    LeCun,
    Zero, // Pour debug seulement
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerConfig {
    pub input_size: usize,
    pub output_size: usize,
    pub activation: Activation,
    pub weight_init: WeightInit,
}

/// Single layer of a neural network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub weights: Array2<f64>,
    pub biases: Array1<f64>,
    pub activation: Activation,

    // Cache pour backprop
    #[serde(skip)]
    last_input: Option<Array1<f64>>,
    #[serde(skip)]
    last_weighted_sum: Option<Array1<f64>>,
    #[serde(skip)]
    last_output: Option<Array1<f64>>,
}

impl Layer {
    pub fn new(config: LayerConfig) -> Self {
        let (weights, biases) =
            Self::initialize_weights(config.input_size, config.output_size, config.weight_init);

        Self {
            weights,
            biases,
            activation: config.activation,
            last_input: None,
            last_weighted_sum: None,
            last_output: None,
        }
    }

    fn initialize_weights(
        input_size: usize,
        output_size: usize,
        init_method: WeightInit,
    ) -> (Array2<f64>, Array1<f64>) {
        let std_dev = match init_method {
            WeightInit::Xavier => (2.0 / (input_size + output_size) as f64).sqrt(),
            WeightInit::He => (2.0 / input_size as f64).sqrt(),
            WeightInit::LeCun => (1.0 / input_size as f64).sqrt(),
            WeightInit::Zero => {
                return (
                    Array2::zeros((output_size, input_size)),
                    Array1::zeros(output_size),
                );
            }
        };

        let normal = Normal::new(0.0, std_dev).unwrap();
        let mut rng = rand::thread_rng();

        let weights = Array2::from_shape_fn((output_size, input_size), |_| normal.sample(&mut rng));

        let biases = Array1::zeros(output_size);

        (weights, biases)
    }

    pub fn forward(&mut self, input: &Array1<f64>) -> Result<Array1<f64>, NetworkError> {
        println!("Input dimensions: {}", input.len());
        println!("Expected dimensions: {}", self.weights.ncols());
        if input.len() != self.weights.ncols() {
            return Err(NetworkError::DimensionMismatch {
                expected: self.weights.ncols(),
                actual: input.len(),
            });
        }

        println!("Weights: {:?}", self.weights);
        println!("Biases: {:?}", self.biases);

        // Cache input pour backprop
        self.last_input = Some(input.clone());

        // Weighted sum
        let weighted_sum = self.weights.dot(input) + &self.biases;
        println!("Weighted sum: {:?}", weighted_sum);
        self.last_weighted_sum = Some(weighted_sum.clone());

        // Apply activation
        let output = weighted_sum.mapv(|x| self.activation.apply(x));
        println!("Output after activation: {:?}", output);
        self.last_output = Some(output.clone());

        Ok(output)
    }

    pub fn backward(
        &mut self,
        output_gradient: &Array1<f64>,
        learning_rate: f64,
    ) -> Result<Array1<f64>, NetworkError> {
        let input = self.last_input.as_ref().ok_or_else(|| {
            NetworkError::InvalidConfig("Must call forward() before backward()".to_string())
        })?;

        let weighted_sum = self.last_weighted_sum.as_ref().ok_or_else(|| {
            NetworkError::InvalidConfig("Must call forward() before backward()".to_string())
        })?;

        // Apply activation derivative
        let activation_grad = weighted_sum.mapv(|x| self.activation.derivative(x));
        let delta = output_gradient * &activation_grad;

        // Calculate gradients
        let weight_gradient = delta
            .clone()
            .insert_axis(Axis(1))
            .dot(&input.clone().insert_axis(Axis(0)));

        let bias_gradient = delta.clone();

        // Calculate input gradient for previous layer (BEFORE update)
        let input_gradient = self.weights.t().dot(&delta);

        // Update parameters
        self.weights = &self.weights - learning_rate * &weight_gradient;
        self.biases = &self.biases - learning_rate * &bias_gradient;

        Ok(input_gradient)
    }

    pub fn update(&mut self, tokens: &[usize]) -> Result<(), NetworkError> {
        println!("Updating weights with {} tokens", tokens.len());
        // Implémentation fictive : mise à jour des poids des couches
        for token in tokens {
            // Exemple fictif de mise à jour
            self.weights.index_axis_mut(Axis(0), *token).fill(0.5);
        }
        Ok(())
    }
}

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
        // Charger un modèle NeuralNetwork depuis un fichier (implémentation simplifiée)
        let data = std::fs::read_to_string(path)
            .map_err(|e| NetworkError::InvalidConfig(e.to_string()))?;
        let layers: Vec<Layer> =
            serde_json::from_str(&data).map_err(|e| NetworkError::InvalidConfig(e.to_string()))?;
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
        // Implémentation fictive : mise à jour des poids des couches
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

// Alias pour compatibilité avec l'ancien code
pub type SimpleNeuralNet = NeuralNetwork;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_forward() {
        let config = LayerConfig {
            input_size: 3,
            output_size: 2,
            activation: Activation::ReLU,
            weight_init: WeightInit::He,
        };

        let mut layer = Layer::new(config);
        let input = Array1::from_vec(vec![1.0, 2.0, 3.0]);

        let output = layer.forward(&input).unwrap();
        assert_eq!(output.len(), 2);
    }

    #[test]
    fn test_network_creation() {
        let configs = vec![
            LayerConfig {
                input_size: 4,
                output_size: 8,
                activation: Activation::ReLU,
                weight_init: WeightInit::He,
            },
            LayerConfig {
                input_size: 8,
                output_size: 1,
                activation: Activation::Sigmoid,
                weight_init: WeightInit::Xavier,
            },
        ];

        let network = NeuralNetwork::new(configs).unwrap();
        assert_eq!(network.input_size(), 4);
        assert_eq!(network.output_size(), 1);
    }

    #[test]
    fn test_forward_backward() {
        let configs = vec![
            LayerConfig {
                input_size: 2,
                output_size: 3,
                activation: Activation::ReLU,
                weight_init: WeightInit::He,
            },
            LayerConfig {
                input_size: 3,
                output_size: 1,
                activation: Activation::Linear,
                weight_init: WeightInit::Xavier,
            },
        ];

        let mut network = NeuralNetwork::new(configs).unwrap();
        let input = Array1::from_vec(vec![1.0, 2.0]);
        let target = Array1::from_vec(vec![1.0]);

        let output = network.forward(&input).unwrap();
        assert_eq!(output.len(), 1);

        let loss = network.backward(&target, 0.01).unwrap();
        assert!(loss >= 0.0);
    }
}
