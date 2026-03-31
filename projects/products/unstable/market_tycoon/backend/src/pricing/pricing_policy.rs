use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingPolicy {
    pub markup_percent: u64,
    pub discount_threshold: u64,
    pub discount_percent: u64,
}

impl Default for PricingPolicy {
    fn default() -> Self {
        Self {
            markup_percent: 30,
            discount_threshold: 100,
            discount_percent: 10,
        }
    }
}
