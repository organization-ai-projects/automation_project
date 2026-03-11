use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyType {
    ContentFilter,
    SafetyCheck,
    FormatValidation,
    LengthLimit(usize),
    Custom(String),
}
