use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DrawdownRisk {
    pub drawdown_from_purchase_pct: f64,
    pub drawdown_from_recent_peak_pct: Option<f64>,
    pub score: f64,
}

impl DrawdownRisk {
    pub fn compute(drawdown_from_purchase: f64, drawdown_from_peak: Option<f64>) -> Self {
        let purchase_score = (-drawdown_from_purchase).min(1.0).max(0.0);
        let peak_score = drawdown_from_peak
            .map(|d| (-d).min(1.0).max(0.0))
            .unwrap_or(0.0);
        let score = (purchase_score * 0.6 + peak_score * 0.4).min(1.0);
        Self {
            drawdown_from_purchase_pct: drawdown_from_purchase,
            drawdown_from_recent_peak_pct: drawdown_from_peak,
            score,
        }
    }
}
