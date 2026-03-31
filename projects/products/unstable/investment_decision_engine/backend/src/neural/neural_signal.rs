use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeuralSignalType {
    DeclineIsPanicDriven,
    DeclineIsFundamental,
    NarrativeOvershoot,
    ThesisIntact,
    ThesisBroken,
    AccumulationOpportunity,
    ExitWarning,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NeuralSignal {
    pub signal_type: NeuralSignalType,
    pub confidence: f64,
    pub explanation: String,
}

impl NeuralSignal {
    pub fn new(
        signal_type: NeuralSignalType,
        confidence: f64,
        explanation: impl Into<String>,
    ) -> Self {
        Self {
            signal_type,
            confidence,
            explanation: explanation.into(),
        }
    }
}
