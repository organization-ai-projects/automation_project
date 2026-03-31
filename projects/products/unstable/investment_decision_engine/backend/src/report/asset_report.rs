use serde::{Deserialize, Serialize};

use crate::assets::AssetProfile;
use crate::config::{EngineConfig, FeatureGateConfig};
use crate::decision::DecisionSummary;
use crate::market_data::MarketSnapshot;
use crate::report::RunHash;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssetReport {
    pub asset_ticker: String,
    pub snapshot_date: String,
    pub current_price: f64,
    pub decision: Option<DecisionSummary>,
    pub run_hash: RunHash,
    pub recommendation_enabled: bool,
}

impl AssetReport {
    pub fn generate(
        asset: &AssetProfile,
        market: &MarketSnapshot,
        _config: &EngineConfig,
        gate: &FeatureGateConfig,
    ) -> Self {
        let hash_input = format!(
            "{}:{}:{}",
            asset.id.canonical_key(),
            market.snapshot_date,
            market.current_price,
        );
        let run_hash = RunHash::compute(&hash_input);

        Self {
            asset_ticker: asset.id.ticker.clone(),
            snapshot_date: market.snapshot_date.clone(),
            current_price: market.current_price,
            decision: None,
            run_hash,
            recommendation_enabled: gate.is_recommendation_allowed(),
        }
    }
}
