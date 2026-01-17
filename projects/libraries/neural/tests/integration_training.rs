// projects/libraries/neural/tests/integration_training.rs
use neural::{
    generation::{GenerationConfig, code_generator::CodeGenerator},
    network::neural_net::{Activation, LayerConfig, NeuralNetwork, WeightInit},
    tokenization::RustTokenizer,
};

#[test]
fn test_code_generator_training() {
    // Mock objects for NeuralNetwork and RustTokenizer
    let layers = vec![
        LayerConfig {
            input_size: 4,
            output_size: 8,
            activation: Activation::ReLU,
            weight_init: WeightInit::Xavier,
        },
        LayerConfig {
            input_size: 8,
            output_size: 4,
            activation: Activation::Sigmoid,
            weight_init: WeightInit::He,
        },
    ];
    let mock_model = NeuralNetwork::new(layers).unwrap();
    let mock_tokenizer = RustTokenizer::new(vec![
        "<PAD>".to_string(),
        "<EOS>".to_string(),
        "<BOS>".to_string(),
        "<UNK>".to_string(),
    ]);
    let config = GenerationConfig::default();

    let mut generator = CodeGenerator::new(mock_model, mock_tokenizer, config);

    let training_data = vec![
        "fn add(a: i32, b: i32) -> i32 { a + b }".to_string(),
        "fn subtract(a: i32, b: i32) -> i32 { a - b }".to_string(),
    ];

    let result = generator.train(training_data);

    assert!(result.is_ok());
}
