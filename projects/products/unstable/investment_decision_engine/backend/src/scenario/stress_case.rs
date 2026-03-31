use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StressCase {
    pub label: String,
    pub price_change_pct: f64,
    pub revenue_change_pct: f64,
    pub probability: f64,
}

impl StressCase {
    pub fn new(
        label: impl Into<String>,
        price_change_pct: f64,
        revenue_change_pct: f64,
        probability: f64,
    ) -> Self {
        Self {
            label: label.into(),
            price_change_pct,
            revenue_change_pct,
            probability,
        }
    }
}
