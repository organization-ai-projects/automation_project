use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PricePoint {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

impl PricePoint {
    pub fn new(date: impl Into<String>, open: f64, high: f64, low: f64, close: f64) -> Self {
        Self {
            date: date.into(),
            open,
            high,
            low,
            close,
        }
    }
}
