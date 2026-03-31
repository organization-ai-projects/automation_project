use crate::neural::neural_signal::{NeuralSignal, NeuralSignalType};

#[test]
fn new_creates_signal() {
    let signal = NeuralSignal::new(NeuralSignalType::DeclineIsPanicDriven, 0.85, "High volume selloff without fundamental catalyst");
    assert_eq!(signal.signal_type, NeuralSignalType::DeclineIsPanicDriven);
    assert!((signal.confidence - 0.85).abs() < f64::EPSILON);
}

#[test]
fn serialization_roundtrip() {
    let signal = NeuralSignal::new(NeuralSignalType::ThesisIntact, 0.9, "Fundamentals unchanged");
    let json = common_json::to_json_string(&signal).unwrap();
    let restored: NeuralSignal = common_json::from_str(&json).unwrap();
    assert_eq!(signal, restored);
}
