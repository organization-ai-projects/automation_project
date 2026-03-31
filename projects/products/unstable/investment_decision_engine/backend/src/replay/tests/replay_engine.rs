use crate::assets::{AssetId, AssetProfile};
use crate::config::{EngineConfig, FeatureGateConfig};
use crate::history::thesis_change::ThesisDirection;
use crate::market_data::{MarketSnapshot, PriceHistory, PricePoint};
use crate::portfolio::{CostBasis, Position, UnrealizedPnl};
use crate::replay::{ReplayEngine, ReplayFile};
use crate::risk::{ConcentrationRisk, DrawdownRisk, RiskScore, ThesisBreakRisk, ValuationRisk};
use crate::sentiment::sentiment_engine::SentimentLabel;

fn sample_replay() -> ReplayFile {
    let asset = AssetProfile::new(AssetId::new("AAPL"), "Apple Inc.");
    let history = PriceHistory::new(
        "AAPL",
        vec![PricePoint::new("2025-01-15", 150.0, 155.0, 148.0, 152.0)],
    );
    let market = MarketSnapshot::new("AAPL", 152.0, history, "2025-01-15");
    let pos = Position::new(AssetId::new("AAPL"), 10.0, CostBasis::new(140.0, 1400.0));
    let pnl = UnrealizedPnl::compute(&pos, 152.0);
    let dd = DrawdownRisk::compute(0.0, Some(0.0));
    let cc = ConcentrationRisk::compute(0.15);
    let vr = ValuationRisk::compute(Some(25.0), Some(0.04));
    let tb = ThesisBreakRisk::compute(ThesisDirection::Unchanged);
    let risk = RiskScore::compute(dd, cc, vr, tb);

    ReplayFile::new("2025-01-15T10:00:00Z", asset, market, pnl, risk, SentimentLabel::Neutral, EngineConfig::default())
}

#[test]
fn replay_produces_deterministic_output() {
    let replay = sample_replay();
    let config = EngineConfig::default();
    let gate = FeatureGateConfig::from_config(&config);

    let result1 = ReplayEngine::execute(&replay, &config, &gate);
    let result2 = ReplayEngine::execute(&replay, &config, &gate);

    let json1 = common_json::to_json_string(&result1).unwrap();
    let json2 = common_json::to_json_string(&result2).unwrap();
    assert_eq!(json1, json2);
}

#[test]
fn replay_respects_feature_gate() {
    let replay = sample_replay();
    let config = EngineConfig::default();
    let mut gate = FeatureGateConfig::from_config(&config);
    gate.recommendation_output_enabled = false;

    let result = ReplayEngine::execute(&replay, &config, &gate);
    assert!(result.recommendation_gated);
}
