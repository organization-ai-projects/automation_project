// projects/products/unstable/hospital_tycoon/backend/src/economy/pricing.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pricing {
    pub consultation_fee: i64,
    pub treatment_fee: i64,
}

impl Default for Pricing {
    fn default() -> Self {
        Self {
            consultation_fee: 100,
            treatment_fee: 200,
        }
    }
}
