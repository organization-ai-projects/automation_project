use neural::{
    network::{Activation, LayerConfig, WeightInit, neural_network::NeuralNetwork},
    tokenization::RustTokenizer,
};

/// Test result type alias for consistent error handling
pub type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

/// Creates a simple 2-layer neural network for integration testing
/// Input: 4 -> Hidden: 8 (ReLU) -> Output: 4 (Sigmoid)
#[allow(dead_code)]
pub fn create_test_network() -> TestResult<NeuralNetwork> {
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
    NeuralNetwork::new(layers).map_err(Into::into)
}

/// Creates a network with configurable vocab size for code generation
#[allow(dead_code)]
pub fn create_network_with_vocab(vocab_size: usize) -> TestResult<NeuralNetwork> {
    NeuralNetwork::new(vec![LayerConfig {
        input_size: vocab_size,
        output_size: 5,
        activation: Activation::ReLU,
        weight_init: WeightInit::Xavier,
    }])
    .map_err(Into::into)
}

/// Creates a basic RustTokenizer with essential special tokens
#[allow(dead_code)]
pub fn create_basic_tokenizer() -> RustTokenizer {
    RustTokenizer::new(vec![
        "<PAD>".to_string(),
        "<EOS>".to_string(),
        "<BOS>".to_string(),
        "<UNK>".to_string(),
    ])
}

/// Creates a RustTokenizer with a rich vocabulary for code generation
#[allow(dead_code)]
pub fn create_code_tokenizer() -> RustTokenizer {
    RustTokenizer::new(vec![
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
        ";".to_string(),
    ])
}

/// Creates sample Rust training data for code generation tests
#[allow(dead_code)]
pub fn create_sample_training_data() -> Vec<String> {
    vec![
        "fn add(a: i32, b: i32) -> i32 { a + b }".to_string(),
        "fn subtract(a: i32, b: i32) -> i32 { a - b }".to_string(),
    ]
}
