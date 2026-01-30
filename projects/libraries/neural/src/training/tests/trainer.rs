// projects/libraries/neural/src/training/tests/trainer.rs
use crate::training::{Trainer, TrainingConfig};

#[test]
fn test_tokenization() {
    let config = TrainingConfig::default();
    let trainer = Trainer::new(10, 1, config);

    let result = trainer.tokenize("test");
    assert!(result.is_ok());
    assert_eq!(result.expect("tokenize succeeds").len(), 4);
}

#[test]
fn test_parse_data() {
    let config = TrainingConfig::default();
    let trainer = Trainer::new(10, 1, config);

    let data = "hello|1.0\nworld|0.5";
    let examples = trainer.parse_data(data).expect("parse data succeeds");

    assert_eq!(examples.len(), 2);
    assert_eq!(examples[0].target, 1.0);
    assert_eq!(examples[1].target, 0.5);
}
