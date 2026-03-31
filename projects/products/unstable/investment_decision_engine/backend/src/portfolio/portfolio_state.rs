use serde::{Deserialize, Serialize};

use crate::portfolio::Position;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortfolioState {
    pub positions: Vec<Position>,
    pub cash_available: f64,
    pub total_value: f64,
}

impl PortfolioState {
    pub fn new(positions: Vec<Position>, cash_available: f64) -> Self {
        let total_value = positions.iter().map(|p| p.total_cost()).sum::<f64>() + cash_available;
        Self {
            positions,
            cash_available,
            total_value,
        }
    }

    pub fn position_for(&self, ticker: &str) -> Option<&Position> {
        self.positions
            .iter()
            .find(|p| p.asset_id.ticker == ticker)
    }

    pub fn concentration(&self, ticker: &str) -> f64 {
        if self.total_value == 0.0 {
            return 0.0;
        }
        self.position_for(ticker)
            .map(|p| p.total_cost() / self.total_value)
            .unwrap_or(0.0)
    }
}
