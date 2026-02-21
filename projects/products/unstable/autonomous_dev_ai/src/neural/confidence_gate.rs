use serde::{Deserialize, Serialize};

/// Validates a model output's confidence and decides whether to use it or fall back.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceGate {
    /// Overall minimum confidence required to use the neural suggestion.
    pub min_confidence: f64,
}

impl ConfidenceGate {
    pub fn new(min_confidence: f64) -> Self {
        Self { min_confidence }
    }

    /// Returns true if the confidence is sufficient to trust the neural output.
    pub fn passes(&self, confidence: f64) -> bool {
        confidence >= self.min_confidence
    }
}

impl Default for ConfidenceGate {
    fn default() -> Self {
        Self::new(0.7)
    }
}
