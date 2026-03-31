use serde::{Deserialize, Serialize};

use crate::model::good::Good;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerSegment {
    pub name: String,
    pub base_demand: u64,
    pub price_sensitivity: u64,
    pub good: Good,
}
