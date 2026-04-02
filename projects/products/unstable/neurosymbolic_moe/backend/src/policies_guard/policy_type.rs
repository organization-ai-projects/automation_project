//! projects/products/unstable/neurosymbolic_moe/backend/src/policies_guard/mod.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyType {
    ContentFilter,
    SafetyCheck,
    FormatValidation,
    LengthLimit(usize),
    Custom(String),
}
