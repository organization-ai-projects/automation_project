use serde::{Deserialize, Serialize};

use crate::config::{EngineConfig, FeatureGateConfig};
use crate::market_data::MarketSnapshot;
use crate::portfolio::PortfolioState;
use crate::report::RunHash;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortfolioReport {
    pub total_positions: usize,
    pub total_value: f64,
    pub cash_available: f64,
    pub run_hash: RunHash,
    pub recommendation_enabled: bool,
}

impl PortfolioReport {
    pub fn generate(
        portfolio: &PortfolioState,
        market: &MarketSnapshot,
        _config: &EngineConfig,
        gate: &FeatureGateConfig,
    ) -> Self {
        let hash_input = format!(
            "portfolio:{}:{}:{}",
            portfolio.positions.len(),
            portfolio.total_value,
            market.snapshot_date,
        );
        let run_hash = RunHash::compute(&hash_input);

        Self {
            total_positions: portfolio.positions.len(),
            total_value: portfolio.total_value,
            cash_available: portfolio.cash_available,
            run_hash,
            recommendation_enabled: gate.is_recommendation_allowed(),
        }
    }
}
