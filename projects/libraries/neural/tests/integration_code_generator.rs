// projects/libraries/neural/tests/integration_code_generator.rs
use neural::{
    generation::{GenerationConfig, code_generator::CodeGenerator},
    network::{Activation, LayerConfig, WeightInit, neural_network::NeuralNetwork},
    tokenization::RustTokenizer,
};

#[test]
fn test_code_generator_integration() {
    // Mock objects for NeuralNetwork and RustTokenizer
    let mock_tokenizer = RustTokenizer::new(vec![
        "fn".to_string(),
        "main".to_string(),
        "(".to_string(),
        ")".to_string(),
        "{".to_string(),
        "}".to_string(),
        "println".to_string(),
        "!".to_string(),
        "\"".to_string(),
        "Hello".to_string(),
        ",".to_string(),
        "world".to_string(),
        "\"".to_string(),
        ";".to_string(),
    ]);

    let vocab_size = mock_tokenizer.vocab_size();
    let mock_model = NeuralNetwork::new(vec![LayerConfig {
        input_size: vocab_size,
        output_size: 5,
        activation: Activation::ReLU,
        weight_init: WeightInit::Xavier,
    }])
    .expect("network init");

    let config = GenerationConfig::default();
    let mut generator = CodeGenerator::new(mock_model, mock_tokenizer, config);

    let prompt = "fn main() { println!(\"Hello, world!\"); }";
    let result = generator.generate(prompt);

    assert!(result.is_ok());
    let generated_code = result.expect("generation succeeds");
    assert!(!generated_code.is_empty());
}
