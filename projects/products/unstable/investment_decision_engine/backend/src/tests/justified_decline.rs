use crate::assets::AssetId;
use crate::config::FeatureGateConfig;
use crate::decision::{CandidateAction, DecisionEngine};
use crate::history::thesis_change::ThesisDirection;
use crate::portfolio::{CostBasis, Position, UnrealizedPnl};
use crate::risk::{ConcentrationRisk, DrawdownRisk, RiskScore, ThesisBreakRisk, ValuationRisk};
use crate::sentiment::sentiment_engine::SentimentLabel;

#[test]
fn justified_decline_recommends_sell() {
    let pos = Position::new(AssetId::new("XYZ"), 100.0, CostBasis::new(50.0, 5000.0));
    let pnl = UnrealizedPnl::compute(&pos, 30.0);

    let dd = DrawdownRisk::compute(-0.4, Some(-0.5));
    let cc = ConcentrationRisk::compute(0.35);
    let vr = ValuationRisk::compute(Some(55.0), Some(0.005));
    let tb = ThesisBreakRisk::compute(ThesisDirection::Broken);
    let risk = RiskScore::compute(dd, cc, vr, tb);

    let gate = FeatureGateConfig::default();
    let summary = DecisionEngine::synthesize(&risk, &pnl, &SentimentLabel::Bearish, &gate);

    assert_eq!(
        summary.recommended_action,
        CandidateAction::Sell,
        "justified decline with broken thesis should recommend sell"
    );
    assert!(!summary.primary_reasons.is_empty());
    assert!(!summary.principal_risks.is_empty());
}
