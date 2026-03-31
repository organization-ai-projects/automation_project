use serde::{Deserialize, Serialize};

use crate::market_data::PricePoint;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriceHistory {
    pub ticker: String,
    pub points: Vec<PricePoint>,
}

impl PriceHistory {
    pub fn new(ticker: impl Into<String>, points: Vec<PricePoint>) -> Self {
        Self {
            ticker: ticker.into(),
            points,
        }
    }

    pub fn latest_close(&self) -> Option<f64> {
        self.points.last().map(|p| p.close)
    }

    pub fn recent_high(&self, n: usize) -> Option<f64> {
        self.points
            .iter()
            .rev()
            .take(n)
            .map(|p| p.high)
            .reduce(f64::max)
    }

    pub fn drawdown_from_recent_peak(&self, n: usize) -> Option<f64> {
        let peak = self.recent_high(n)?;
        let current = self.latest_close()?;
        if peak > 0.0 {
            Some((current - peak) / peak)
        } else {
            None
        }
    }
}
