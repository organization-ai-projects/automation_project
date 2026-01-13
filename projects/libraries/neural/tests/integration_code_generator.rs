use neural::{
    generation::{GenerationConfig, code_generator::CodeGenerator},
    network::neural_net::{Activation, LayerConfig, NeuralNetwork, WeightInit},
    tokenization::RustTokenizer,
};

#[test]
fn test_code_generator_integration() {
    // Mock objects for NeuralNetwork and RustTokenizer
    let mock_model = NeuralNetwork::new(vec![LayerConfig {
        input_size: 10,
        output_size: 5,
        activation: Activation::ReLU,
        weight_init: WeightInit::Xavier,
    }])
    .unwrap(); // Ajout d'une configuration valide
    let mock_tokenizer = RustTokenizer::new(vec![
        "<PAD>".to_string(),
        "<EOS>".to_string(),
        "<BOS>".to_string(),
        "<UNK>".to_string(),
    ]);
    let config = GenerationConfig::default();

    let mut generator = CodeGenerator::new(mock_model, mock_tokenizer, config);

    let prompt = "fn main() { println!(\"Hello, world!\"); }";
    let result = generator.generate(prompt);

    assert!(result.is_ok());
    let generated_code = result.unwrap();
    assert!(!generated_code.is_empty());
}
