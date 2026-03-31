use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConcentrationRisk {
    pub portfolio_weight: f64,
    pub score: f64,
}

impl ConcentrationRisk {
    pub fn compute(portfolio_weight: f64) -> Self {
        let score = if portfolio_weight > 0.5 {
            1.0
        } else if portfolio_weight > 0.3 {
            0.7
        } else if portfolio_weight > 0.15 {
            0.4
        } else {
            0.1
        };
        Self {
            portfolio_weight,
            score,
        }
    }
}
