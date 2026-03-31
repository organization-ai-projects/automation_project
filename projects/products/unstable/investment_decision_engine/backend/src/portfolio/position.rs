use serde::{Deserialize, Serialize};

use crate::assets::AssetId;
use crate::portfolio::CostBasis;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub asset_id: AssetId,
    pub shares: f64,
    pub cost_basis: CostBasis,
}

impl Position {
    pub fn new(asset_id: AssetId, shares: f64, cost_basis: CostBasis) -> Self {
        Self {
            asset_id,
            shares,
            cost_basis,
        }
    }

    pub fn total_cost(&self) -> f64 {
        self.shares * self.cost_basis.average_price
    }
}
