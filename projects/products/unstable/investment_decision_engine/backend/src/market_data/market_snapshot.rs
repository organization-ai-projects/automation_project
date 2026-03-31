use serde::{Deserialize, Serialize};

use crate::market_data::{PriceHistory, VolumeHistory};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketSnapshot {
    pub ticker: String,
    pub current_price: f64,
    pub price_history: PriceHistory,
    pub volume_history: Option<VolumeHistory>,
    pub snapshot_date: String,
}

impl MarketSnapshot {
    pub fn new(
        ticker: impl Into<String>,
        current_price: f64,
        price_history: PriceHistory,
        snapshot_date: impl Into<String>,
    ) -> Self {
        Self {
            ticker: ticker.into(),
            current_price,
            price_history,
            volume_history: None,
            snapshot_date: snapshot_date.into(),
        }
    }
}
