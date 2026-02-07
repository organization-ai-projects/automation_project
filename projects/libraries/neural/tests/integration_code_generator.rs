// projects/libraries/neural/tests/integration_code_generator.rs
mod helpers;

use neural::generation::{GenerationConfig, code_generator::CodeGenerator};
use helpers::{create_network_with_vocab, create_code_tokenizer};

#[test]
fn test_code_generator_integration() {
    let mock_tokenizer = create_code_tokenizer();
    let vocab_size = mock_tokenizer.vocab_size();
    let mock_model = create_network_with_vocab(vocab_size).expect("network creation should succeed");

    let config = GenerationConfig::default();
    let mut generator = CodeGenerator::new(mock_model, mock_tokenizer, config);

    let prompt = "fn main() { println!(\"Hello, world!\"); }";
    let result = generator.generate(prompt);

    assert!(result.is_ok(), "code generation should succeed");
    let generated_code = result.expect("generation should return valid code");
    assert!(!generated_code.is_empty(), "generated code should not be empty");
    assert!(generated_code.len() > 0, "generated code should have non-zero length");
}
