use crate::network::{Activation, LayerConfig, WeightInit, neural_network::NeuralNetwork};

/// Test result type alias for consistent error handling
pub(crate) type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

/// Creates a simple 2-layer neural network for testing
/// Input: 4 -> Hidden: 8 (ReLU) -> Output: 1 (Sigmoid)
pub(crate) fn create_simple_network() -> TestResult<NeuralNetwork> {
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
    NeuralNetwork::new(configs).map_err(Into::into)
}

/// Creates a small 2-layer neural network for quick testing
/// Input: 2 -> Hidden: 3 (ReLU) -> Output: 1 (Linear)
pub(crate) fn create_small_network() -> TestResult<NeuralNetwork> {
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
    NeuralNetwork::new(configs).map_err(Into::into)
}

/// Creates a layer configuration for testing
pub(crate) fn create_test_layer_config(
    input_size: usize,
    output_size: usize,
    activation: Activation,
) -> LayerConfig {
    LayerConfig {
        input_size,
        output_size,
        activation,
        weight_init: WeightInit::He,
    }
}
