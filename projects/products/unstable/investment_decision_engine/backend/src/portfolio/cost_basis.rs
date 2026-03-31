use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CostBasis {
    pub average_price: f64,
    pub total_invested: f64,
}

impl CostBasis {
    pub fn new(average_price: f64, total_invested: f64) -> Self {
        Self {
            average_price,
            total_invested,
        }
    }

    pub fn from_single_purchase(price: f64, shares: f64) -> Self {
        Self {
            average_price: price,
            total_invested: price * shares,
        }
    }
}
