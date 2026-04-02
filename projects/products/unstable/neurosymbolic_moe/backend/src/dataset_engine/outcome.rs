use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Outcome {
    Success,
    Failure,
    Partial,
    Unknown,
}

impl PartialEq for Outcome {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Eq for Outcome {}
