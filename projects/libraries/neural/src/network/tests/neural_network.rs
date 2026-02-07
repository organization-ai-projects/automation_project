// projects/libraries/neural/src/network/tests/neural_network.rs
use ndarray::Array1;

use crate::network::Activation;
use crate::tests::test_helpers::{
    create_simple_network, create_small_network, create_test_layer_config,
};

#[test]
fn test_layer_forward() {
    let config = create_test_layer_config(3, 2, Activation::ReLU);

    let mut layer = crate::network::Layer::new(config);
    let input = Array1::from_vec(vec![1.0, 2.0, 3.0]);

    let output = layer.forward(&input).expect("forward should succeed");
    assert_eq!(output.len(), 2, "output should have correct dimension");
}

#[test]
fn test_network_creation() {
    let network = create_simple_network().expect("network creation should succeed");

    assert_eq!(
        network.input_size(),
        4,
        "input size should match configuration"
    );
    assert_eq!(
        network.output_size(),
        1,
        "output size should match configuration"
    );
}

#[test]
fn test_forward_backward() {
    let mut network = create_small_network().expect("network creation should succeed");
    let input = Array1::from_vec(vec![1.0, 2.0]);
    let target = Array1::from_vec(vec![1.0]);

    let output = network
        .forward(&input)
        .expect("forward pass should succeed");
    assert_eq!(output.len(), 1, "output should have correct dimension");
    assert!(output[0].is_finite(), "output should be a finite number");

    let loss = network
        .backward(&target, 0.01)
        .expect("backward pass should succeed");
    assert!(loss >= 0.0, "loss should be non-negative");
    assert!(loss.is_finite(), "loss should be finite");
}
