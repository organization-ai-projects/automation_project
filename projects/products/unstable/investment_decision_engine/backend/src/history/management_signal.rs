use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalDirection {
    Positive,
    Negative,
    Neutral,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManagementSignal {
    pub date: String,
    pub direction: SignalDirection,
    pub source: String,
    pub description: String,
}

impl ManagementSignal {
    pub fn new(
        date: impl Into<String>,
        direction: SignalDirection,
        source: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            date: date.into(),
            direction,
            source: source.into(),
            description: description.into(),
        }
    }
}
