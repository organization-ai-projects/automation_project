// projects/libraries/neural/tests/integration_training.rs
mod helpers;

use helpers::{create_basic_tokenizer, create_sample_training_data, create_test_network};
use neural::generation::{GenerationConfig, code_generator::CodeGenerator};

#[test]
fn test_code_generator_training() {
    let mock_model = create_test_network().expect("network creation should succeed");
    let mock_tokenizer = create_basic_tokenizer();
    let config = GenerationConfig::default();

    let mut generator = CodeGenerator::new(mock_model, mock_tokenizer, config);
    let training_data = create_sample_training_data();

    let result = generator.train(training_data);

    assert!(result.is_ok(), "training should succeed without errors");
}
