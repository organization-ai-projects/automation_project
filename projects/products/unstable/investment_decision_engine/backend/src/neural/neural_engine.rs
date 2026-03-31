use crate::config::FeatureGateConfig;
use crate::neural::{NeuralInput, NeuralSignal, NeuralSummary};

pub struct NeuralEngine;

impl NeuralEngine {
    pub fn process(input: &NeuralInput, gate: &FeatureGateConfig) -> NeuralSummary {
        if !gate.is_neural_allowed() || !input.has_content() {
            return NeuralSummary::empty(&input.ticker);
        }

        NeuralSummary::empty(&input.ticker)
    }

    pub fn extract_signals(_summary: &NeuralSummary) -> Vec<NeuralSignal> {
        Vec::new()
    }
}
