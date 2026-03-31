use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NarrativeDirection {
    BullishToNeutral,
    BullishToBearish,
    NeutralToBullish,
    NeutralToBearish,
    BearishToNeutral,
    BearishToBullish,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NarrativeShift {
    pub date: String,
    pub direction: NarrativeDirection,
    pub description: String,
    pub confidence: f64,
}

impl NarrativeShift {
    pub fn new(
        date: impl Into<String>,
        direction: NarrativeDirection,
        description: impl Into<String>,
        confidence: f64,
    ) -> Self {
        Self {
            date: date.into(),
            direction,
            description: description.into(),
            confidence,
        }
    }

    pub fn is_bearish_shift(&self) -> bool {
        matches!(
            self.direction,
            NarrativeDirection::BullishToBearish
                | NarrativeDirection::NeutralToBearish
                | NarrativeDirection::BullishToNeutral
        )
    }
}
