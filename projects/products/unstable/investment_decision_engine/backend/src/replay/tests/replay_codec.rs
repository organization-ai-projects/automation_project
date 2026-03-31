use crate::assets::{AssetId, AssetProfile};
use crate::config::EngineConfig;
use crate::history::thesis_change::ThesisDirection;
use crate::market_data::{MarketSnapshot, PriceHistory, PricePoint};
use crate::portfolio::{CostBasis, Position, UnrealizedPnl};
use crate::replay::{ReplayCodec, ReplayFile};
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
fn encode_decode_roundtrip() {
    let replay = sample_replay();
    let encoded = ReplayCodec::encode(&replay).unwrap();
    let decoded = ReplayCodec::decode(&encoded).unwrap();
    assert_eq!(replay, decoded);
}

#[test]
fn decode_invalid_json_fails() {
    let result = ReplayCodec::decode("not valid json");
    assert!(result.is_err());
}
