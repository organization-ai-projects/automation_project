use neural::{
    generation::{GenerationConfig, code_generator::CodeGenerator},
    network::neural_net::NeuralNetwork,
    tokenization::RustTokenizer,
};

#[test]
fn test_code_generator_training() {
    // Mock objects for NeuralNetwork and RustTokenizer
    let mock_model = NeuralNetwork::new(vec![]).unwrap();
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
