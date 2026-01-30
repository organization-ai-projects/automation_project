// projects/libraries/neural/src/network/tests/neural_network.rs
use ndarray::Array1;

use crate::network::{Activation, LayerConfig, WeightInit, neural_network::NeuralNetwork};

#[test]
fn test_layer_forward() {
    let config = LayerConfig {
        input_size: 3,
        output_size: 2,
        activation: Activation::ReLU,
        weight_init: WeightInit::He,
    };

    let mut layer = crate::network::Layer::new(config);
    let input = Array1::from_vec(vec![1.0, 2.0, 3.0]);

    let output = layer.forward(&input).expect("forward succeeds");
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

    let network = NeuralNetwork::new(configs).expect("network init");
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

    let mut network = NeuralNetwork::new(configs).expect("network init");
    let input = Array1::from_vec(vec![1.0, 2.0]);
    let target = Array1::from_vec(vec![1.0]);

    let output = network.forward(&input).expect("forward succeeds");
    assert_eq!(output.len(), 1);

    let loss = network.backward(&target, 0.01).expect("backward succeeds");
    assert!(loss >= 0.0);
}
