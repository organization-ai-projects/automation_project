use serde::{Deserialize, Serialize};

use crate::assets::AssetProfile;
use crate::config::EngineConfig;
use crate::market_data::MarketSnapshot;
use crate::portfolio::UnrealizedPnl;
use crate::risk::RiskScore;
use crate::sentiment::sentiment_engine::SentimentLabel;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReplayFile {
    pub version: u32,
    pub timestamp: String,
    pub asset: AssetProfile,
    pub market_snapshot: MarketSnapshot,
    pub pnl: UnrealizedPnl,
    pub risk_score: RiskScore,
    pub sentiment: SentimentLabel,
    pub config: EngineConfig,
}

impl ReplayFile {
    pub fn new(
        timestamp: impl Into<String>,
        asset: AssetProfile,
        market_snapshot: MarketSnapshot,
        pnl: UnrealizedPnl,
        risk_score: RiskScore,
        sentiment: SentimentLabel,
        config: EngineConfig,
    ) -> Self {
        Self {
            version: 1,
            timestamp: timestamp.into(),
            asset,
            market_snapshot,
            pnl,
            risk_score,
            sentiment,
            config,
        }
    }
}
