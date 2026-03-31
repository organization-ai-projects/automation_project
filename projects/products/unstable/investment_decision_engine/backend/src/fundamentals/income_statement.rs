use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncomeStatement {
    pub period: String,
    pub revenue: f64,
    pub gross_profit: f64,
    pub operating_income: f64,
    pub net_income: f64,
    pub eps: f64,
}

impl IncomeStatement {
    pub fn gross_margin(&self) -> Option<f64> {
        if self.revenue > 0.0 {
            Some(self.gross_profit / self.revenue)
        } else {
            None
        }
    }

    pub fn operating_margin(&self) -> Option<f64> {
        if self.revenue > 0.0 {
            Some(self.operating_income / self.revenue)
        } else {
            None
        }
    }

    pub fn net_margin(&self) -> Option<f64> {
        if self.revenue > 0.0 {
            Some(self.net_income / self.revenue)
        } else {
            None
        }
    }
}
