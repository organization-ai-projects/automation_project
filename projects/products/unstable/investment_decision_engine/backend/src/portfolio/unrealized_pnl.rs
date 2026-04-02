use serde::{Deserialize, Serialize};

use crate::portfolio::Position;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnrealizedPnl {
    pub ticker: String,
    pub current_price: f64,
    pub cost_basis_price: f64,
    pub shares: f64,
    pub unrealized_gain_loss: f64,
    pub unrealized_gain_loss_pct: f64,
    pub drawdown_from_purchase: f64,
}

impl UnrealizedPnl {
    pub fn compute(position: &Position, current_price: f64) -> Self {
        let cost = position.cost_basis.average_price;
        let gain_loss = (current_price - cost) * position.shares;
        let gain_loss_pct = if cost > 0.0 {
            (current_price - cost) / cost
        } else {
            0.0
        };
        let drawdown = if cost > 0.0 {
            ((current_price - cost) / cost).min(0.0)
        } else {
            0.0
        };
        Self {
            ticker: position.asset_id.ticker.clone(),
            current_price,
            cost_basis_price: cost,
            shares: position.shares,
            unrealized_gain_loss: gain_loss,
            unrealized_gain_loss_pct: gain_loss_pct,
            drawdown_from_purchase: drawdown,
        }
    }

    pub fn is_loss(&self) -> bool {
        self.unrealized_gain_loss < 0.0
    }

    pub fn is_gain(&self) -> bool {
        self.unrealized_gain_loss > 0.0
    }
}
