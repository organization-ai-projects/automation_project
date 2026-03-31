use crate::assets::AssetId;
use crate::config::FeatureGateConfig;
use crate::decision::{CandidateAction, DecisionEngine};
use crate::history::thesis_change::ThesisDirection;
use crate::portfolio::{CostBasis, Position, UnrealizedPnl};
use crate::risk::{ConcentrationRisk, DrawdownRisk, RiskScore, ThesisBreakRisk, ValuationRisk};
use crate::sentiment::sentiment_engine::SentimentLabel;

#[test]
fn low_risk_bearish_sentiment_intact_thesis_can_buy_more() {
    let pos = Position::new(AssetId::new("GHI"), 30.0, CostBasis::new(200.0, 6000.0));
    let pnl = UnrealizedPnl::compute(&pos, 195.0);

    let dd = DrawdownRisk::compute(-0.025, Some(-0.05));
    let cc = ConcentrationRisk::compute(0.05);
    let vr = ValuationRisk::compute(Some(10.0), Some(0.08));
    let tb = ThesisBreakRisk::compute(ThesisDirection::Strengthened);
    let risk = RiskScore::compute(dd, cc, vr, tb);

    let gate = FeatureGateConfig::default();
    let summary = DecisionEngine::synthesize(&risk, &pnl, &SentimentLabel::Bearish, &gate);

    assert_eq!(
        summary.recommended_action,
        CandidateAction::BuyMore,
        "low risk with bearish sentiment and strong thesis should recommend buy more"
    );
}
