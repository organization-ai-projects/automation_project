use crate::assets::{AssetId, AssetProfile};
use crate::config::{EngineConfig, FeatureGateConfig};
use crate::market_data::{MarketSnapshot, PriceHistory, PricePoint};
use crate::report::AssetReport;

fn sample_report() -> AssetReport {
    let asset = AssetProfile::new(AssetId::new("AAPL"), "Apple Inc.");
    let history = PriceHistory::new("AAPL", vec![PricePoint::new("2025-01-15", 150.0, 155.0, 148.0, 152.0)]);
    let market = MarketSnapshot::new("AAPL", 152.0, history, "2025-01-15");
    let config = EngineConfig::default();
    let gate = FeatureGateConfig::from_config(&config);
    AssetReport::generate(&asset, &market, &config, &gate)
}

#[test]
fn generate_produces_report() {
    let report = sample_report();
    assert_eq!(report.asset_ticker, "AAPL");
    assert!(!report.run_hash.0.is_empty());
}

#[test]
fn deterministic_hash() {
    let r1 = sample_report();
    let r2 = sample_report();
    assert_eq!(r1.run_hash, r2.run_hash);
}

#[test]
fn serialization_roundtrip() {
    let report = sample_report();
    let json = common_json::to_json_string(&report).unwrap();
    let restored: AssetReport = common_json::from_str(&json).unwrap();
    assert_eq!(report, restored);
}
