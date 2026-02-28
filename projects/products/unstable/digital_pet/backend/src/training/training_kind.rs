// projects/products/unstable/digital_pet/backend/src/training/training_kind.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrainingKind {
    Strength,
    Speed,
    Defense,
    Stamina,
}

impl TrainingKind {
    pub fn from_str(s: &str) -> Self {
        match s {
            "strength" => Self::Strength,
            "speed" => Self::Speed,
            "defense" => Self::Defense,
            "stamina" => Self::Stamina,
            _ => Self::Strength,
        }
    }
}
