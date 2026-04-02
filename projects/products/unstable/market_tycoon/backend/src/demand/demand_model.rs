use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemandModel {
    pub elasticity: f64,
    pub base_price_reference: i64,
}

impl Default for DemandModel {
    fn default() -> Self {
        Self {
            elasticity: 1.5,
            base_price_reference: 1000,
        }
    }
}
