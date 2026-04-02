use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CashFlowStatement {
    pub period: String,
    pub operating_cash_flow: f64,
    pub capital_expenditures: f64,
    pub free_cash_flow: f64,
    pub dividends_paid: f64,
}

impl CashFlowStatement {
    pub fn fcf_yield(&self, market_cap: f64) -> Option<f64> {
        if market_cap > 0.0 {
            Some(self.free_cash_flow / market_cap)
        } else {
            None
        }
    }
}
