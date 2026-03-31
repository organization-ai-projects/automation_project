use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RngDraw {
    pub step: u64,
    pub label: String,
    pub value: u64,
    pub range_max: u64,
}

impl RngDraw {
    pub fn new(step: u64, label: &str, value: u64, range_max: u64) -> Self {
        Self {
            step,
            label: label.to_string(),
            value,
            range_max,
        }
    }
}
