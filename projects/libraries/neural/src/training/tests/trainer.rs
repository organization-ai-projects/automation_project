// projects/libraries/neural/src/training/tests/trainer.rs
use crate::training::{Trainer, TrainingConfig};

#[test]
fn test_tokenization() {
    let config = TrainingConfig::default();
    let trainer = Trainer::new(10, 1, config);

    let result = trainer.tokenize("test");
    assert!(result.is_ok(), "tokenization should succeed");

    let tokens = result.expect("tokenization should return valid tokens");
    assert_eq!(tokens.len(), 4, "tokenized output should have 4 tokens");
    assert!(
        tokens
            .iter()
            .all(|&t| t.is_finite() && t >= 0.0 && t <= 1.0),
        "all tokens should be finite and in the range [0.0, 1.0]"
    );
}

#[test]
fn test_parse_data() {
    let config = TrainingConfig::default();
    let trainer = Trainer::new(10, 1, config);

    let data = "hello|1.0\nworld|0.5";
    let examples = trainer
        .parse_data(data)
        .expect("data parsing should succeed");

    assert_eq!(examples.len(), 2, "should parse 2 examples");
    assert_eq!(examples[0].target, 1.0, "first target should be 1.0");
    assert_eq!(examples[1].target, 0.5, "second target should be 0.5");
    assert!(
        examples[0].input.len() > 0,
        "first example should have input tokens"
    );
    assert!(
        examples[1].input.len() > 0,
        "second example should have input tokens"
    );
}
