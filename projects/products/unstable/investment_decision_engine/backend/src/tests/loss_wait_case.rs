use crate::assets::AssetId;
use crate::config::FeatureGateConfig;
use crate::decision::{CandidateAction, DecisionEngine};
use crate::history::thesis_change::ThesisDirection;
use crate::portfolio::{CostBasis, Position, UnrealizedPnl};
use crate::risk::{ConcentrationRisk, DrawdownRisk, RiskScore, ThesisBreakRisk, ValuationRisk};
use crate::sentiment::sentiment_engine::SentimentLabel;

#[test]
fn unrealized_loss_with_intact_thesis_recommends_hold_with_wait() {
    let pos = Position::new(AssetId::new("DEF"), 200.0, CostBasis::new(80.0, 16000.0));
    let pnl = UnrealizedPnl::compute(&pos, 50.0);

    let dd = DrawdownRisk::compute(-0.375, Some(-0.4));
    let cc = ConcentrationRisk::compute(0.20);
    let vr = ValuationRisk::compute(Some(15.0), Some(0.05));
    let tb = ThesisBreakRisk::compute(ThesisDirection::Unchanged);
    let risk = RiskScore::compute(dd, cc, vr, tb);

    let gate = FeatureGateConfig::default();
    let summary = DecisionEngine::synthesize(&risk, &pnl, &SentimentLabel::Neutral, &gate);

    assert_eq!(
        summary.recommended_action,
        CandidateAction::Hold,
        "unrealized loss with intact thesis should recommend hold"
    );
    assert!(
        summary.wait_thesis.is_some(),
        "hold recommendation should include wait thesis"
    );
    let wt = summary.wait_thesis.unwrap();
    assert!(wt.should_wait, "wait thesis should recommend waiting");
}
