use crate::assets::{AssetId, AssetProfile};
use crate::config::{EngineConfig, FeatureGateConfig};
use crate::history::thesis_change::ThesisDirection;
use crate::market_data::{MarketSnapshot, PriceHistory, PricePoint};
use crate::portfolio::{CostBasis, Position, UnrealizedPnl};
use crate::replay::{ReplayCodec, ReplayEngine, ReplayFile};
use crate::risk::{ConcentrationRisk, DrawdownRisk, RiskScore, ThesisBreakRisk, ValuationRisk};
use crate::sentiment::sentiment_engine::SentimentLabel;

fn build_replay() -> ReplayFile {
    let asset = AssetProfile::new(AssetId::new("TEST"), "Test Corp");
    let history = PriceHistory::new(
        "TEST",
        vec![
            PricePoint::new("2025-01-13", 100.0, 105.0, 95.0, 102.0),
            PricePoint::new("2025-01-14", 102.0, 108.0, 100.0, 106.0),
            PricePoint::new("2025-01-15", 106.0, 110.0, 104.0, 107.0),
        ],
    );
    let market = MarketSnapshot::new("TEST", 107.0, history, "2025-01-15");
    let pos = Position::new(AssetId::new("TEST"), 50.0, CostBasis::new(95.0, 4750.0));
    let pnl = UnrealizedPnl::compute(&pos, 107.0);

    let dd = DrawdownRisk::compute(0.0, Some(-0.03));
    let cc = ConcentrationRisk::compute(0.15);
    let vr = ValuationRisk::compute(Some(22.0), Some(0.045));
    let tb = ThesisBreakRisk::compute(ThesisDirection::Unchanged);
    let risk = RiskScore::compute(dd, cc, vr, tb);

    ReplayFile::new(
        "2025-01-15T12:00:00Z",
        asset,
        market,
        pnl,
        risk,
        SentimentLabel::Neutral,
        EngineConfig::default(),
    )
}

#[test]
fn replay_produces_identical_results() {
    let replay = build_replay();
    let config = EngineConfig::default();
    let gate = FeatureGateConfig::from_config(&config);

    let result1 = ReplayEngine::execute(&replay, &config, &gate);
    let result2 = ReplayEngine::execute(&replay, &config, &gate);

    let json1 = common_json::to_json_string(&result1).unwrap();
    let json2 = common_json::to_json_string(&result2).unwrap();
    assert_eq!(json1, json2, "replay must be deterministic");
}

#[test]
fn replay_roundtrip_preserves_equivalence() {
    let replay = build_replay();
    let config = EngineConfig::default();
    let gate = FeatureGateConfig::from_config(&config);

    let encoded = ReplayCodec::encode(&replay).unwrap();
    let decoded = ReplayCodec::decode(&encoded).unwrap();

    let result_original = ReplayEngine::execute(&replay, &config, &gate);
    let result_decoded = ReplayEngine::execute(&decoded, &config, &gate);

    let json_original = common_json::to_json_string(&result_original).unwrap();
    let json_decoded = common_json::to_json_string(&result_decoded).unwrap();
    assert_eq!(
        json_original, json_decoded,
        "replay from decoded file must match original"
    );
}
