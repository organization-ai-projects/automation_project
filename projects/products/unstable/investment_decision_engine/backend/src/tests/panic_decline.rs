use crate::assets::AssetId;
use crate::config::FeatureGateConfig;
use crate::decision::{CandidateAction, DecisionEngine};
use crate::history::thesis_change::ThesisDirection;
use crate::portfolio::{CostBasis, Position, UnrealizedPnl};
use crate::risk::{ConcentrationRisk, DrawdownRisk, RiskScore, ThesisBreakRisk, ValuationRisk};
use crate::sentiment::sentiment_engine::SentimentLabel;

#[test]
fn panic_decline_with_intact_thesis_does_not_sell() {
    let pos = Position::new(AssetId::new("ABC"), 50.0, CostBasis::new(100.0, 5000.0));
    let pnl = UnrealizedPnl::compute(&pos, 65.0);

    let dd = DrawdownRisk::compute(-0.35, Some(-0.4));
    let cc = ConcentrationRisk::compute(0.10);
    let vr = ValuationRisk::compute(Some(12.0), Some(0.07));
    let tb = ThesisBreakRisk::compute(ThesisDirection::Unchanged);
    let risk = RiskScore::compute(dd, cc, vr, tb);

    let gate = FeatureGateConfig::default();
    let summary = DecisionEngine::synthesize(&risk, &pnl, &SentimentLabel::Bearish, &gate);

    assert_ne!(
        summary.recommended_action,
        CandidateAction::Sell,
        "panic-driven decline with intact thesis should not recommend sell"
    );
}
