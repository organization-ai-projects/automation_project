use crate::neural::NeuralInput;

#[test]
fn empty_input_has_no_content() {
    let input = NeuralInput::new("AAPL");
    assert!(!input.has_content());
}

#[test]
fn input_with_history_has_content() {
    let mut input = NeuralInput::new("AAPL");
    input.company_history_text = Some("Founded in 1976".to_string());
    assert!(input.has_content());
}

#[test]
fn serialization_roundtrip() {
    let input = NeuralInput::new("GOOG");
    let json = common_json::to_json_string(&input).unwrap();
    let restored: NeuralInput = common_json::from_str(&json).unwrap();
    assert_eq!(input, restored);
}
