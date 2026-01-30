// projects/libraries/neural/src/network/layer.rs
use ndarray::{Array1, Array2, Axis};
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};

use crate::network::{
    Activation, LayerConfig, network_error::NetworkError, weight_init::WeightInit,
};

/// Single layer of a neural network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub weights: Array2<f64>,
    pub biases: Array1<f64>,
    pub activation: Activation,

    // Cache for backpropagation
    #[serde(skip)]
    pub(crate) last_input: Option<Array1<f64>>,
    #[serde(skip)]
    pub(crate) last_weighted_sum: Option<Array1<f64>>,
    #[serde(skip)]
    pub(crate) last_output: Option<Array1<f64>>,
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

        let normal = Normal::new(0.0, std_dev).expect("valid normal distribution");
        let mut rng = rand::rng();

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

        // Cache input for backpropagation
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
        // Placeholder implementation: updating layer weights
        for token in tokens {
            // Example placeholder update
            self.weights.index_axis_mut(Axis(0), *token).fill(0.5);
        }
        Ok(())
    }
}
