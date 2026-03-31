use crate::config::FeatureGateConfig;
use crate::neural::{NeuralEngine, NeuralInput};

#[test]
fn disabled_gate_returns_empty_summary() {
    let input = NeuralInput::new("AAPL");
    let mut gate = FeatureGateConfig::default();
    gate.neural_assistance_enabled = false;
    let summary = NeuralEngine::process(&input, &gate);
    assert!(summary.company_summary.is_none());
}

#[test]
fn empty_input_returns_empty_summary() {
    let input = NeuralInput::new("AAPL");
    let mut gate = FeatureGateConfig::default();
    gate.neural_assistance_enabled = true;
    let summary = NeuralEngine::process(&input, &gate);
    assert!(summary.company_summary.is_none());
}

#[test]
fn extract_signals_returns_empty_for_empty_summary() {
    let summary = crate::neural::NeuralSummary::empty("AAPL");
    let signals = NeuralEngine::extract_signals(&summary);
    assert!(signals.is_empty());
}
