use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BalanceSheet {
    pub period: String,
    pub total_assets: f64,
    pub total_liabilities: f64,
    pub total_equity: f64,
    pub cash_and_equivalents: f64,
    pub total_debt: f64,
}

impl BalanceSheet {
    pub fn debt_to_equity(&self) -> Option<f64> {
        if self.total_equity > 0.0 {
            Some(self.total_debt / self.total_equity)
        } else {
            None
        }
    }

    pub fn net_debt(&self) -> f64 {
        self.total_debt - self.cash_and_equivalents
    }
}
