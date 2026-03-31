use crate::assets::AssetId;
use crate::config::{EngineConfig, FeatureGateConfig};
use crate::market_data::{MarketSnapshot, PriceHistory, PricePoint};
use crate::portfolio::{CostBasis, PortfolioState, Position};
use crate::report::PortfolioReport;

#[test]
fn generate_portfolio_report() {
    let positions = vec![Position::new(
        AssetId::new("AAPL"),
        10.0,
        CostBasis::new(150.0, 1500.0),
    )];
    let state = PortfolioState::new(positions, 500.0);
    let history = PriceHistory::new(
        "AAPL",
        vec![PricePoint::new("2025-01-15", 150.0, 155.0, 148.0, 152.0)],
    );
    let market = MarketSnapshot::new("AAPL", 152.0, history, "2025-01-15");
    let config = EngineConfig::default();
    let gate = FeatureGateConfig::from_config(&config);

    let report = PortfolioReport::generate(&state, &market, &config, &gate);
    assert_eq!(report.total_positions, 1);
    assert!(!report.run_hash.0.is_empty());
}
