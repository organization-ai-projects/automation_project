use crate::assets::AssetId;
use crate::config::FeatureGateConfig;
use crate::decision::{CandidateAction, DecisionEngine};
use crate::history::thesis_change::ThesisDirection;
use crate::portfolio::{CostBasis, Position, UnrealizedPnl};
use crate::risk::{ConcentrationRisk, DrawdownRisk, RiskScore, ThesisBreakRisk, ValuationRisk};
use crate::sentiment::sentiment_engine::SentimentLabel;

fn make_risk(thesis_dir: ThesisDirection) -> RiskScore {
    let dd = DrawdownRisk::compute(-0.1, Some(-0.05));
    let cc = ConcentrationRisk::compute(0.2);
    let vr = ValuationRisk::compute(Some(20.0), Some(0.04));
    let tb = ThesisBreakRisk::compute(thesis_dir);
    RiskScore::compute(dd, cc, vr, tb)
}

fn make_pnl(cost: f64, current: f64) -> UnrealizedPnl {
    let pos = Position::new(
        AssetId::new("TEST"),
        10.0,
        CostBasis::new(cost, cost * 10.0),
    );
    UnrealizedPnl::compute(&pos, current)
}

#[test]
fn gated_recommendation_when_disabled() {
    let risk = make_risk(ThesisDirection::Unchanged);
    let pnl = make_pnl(100.0, 110.0);
    let mut gate = FeatureGateConfig::default();
    gate.recommendation_output_enabled = false;
    let summary = DecisionEngine::synthesize(&risk, &pnl, &SentimentLabel::Neutral, &gate);
    assert!(summary.recommendation_gated);
}

#[test]
fn broken_thesis_high_risk_recommends_sell() {
    let risk = make_risk(ThesisDirection::Broken);
    let pnl = make_pnl(100.0, 110.0);
    let gate = FeatureGateConfig::default();
    let summary = DecisionEngine::synthesize(&risk, &pnl, &SentimentLabel::Bearish, &gate);
    assert_eq!(summary.recommended_action, CandidateAction::Sell);
    assert!(!summary.primary_reasons.is_empty());
}

#[test]
fn loss_with_intact_thesis_recommends_hold() {
    let risk = make_risk(ThesisDirection::Unchanged);
    let pnl = make_pnl(100.0, 60.0);
    let gate = FeatureGateConfig::default();
    let summary = DecisionEngine::synthesize(&risk, &pnl, &SentimentLabel::Neutral, &gate);
    assert_eq!(summary.recommended_action, CandidateAction::Hold);
    assert!(summary.wait_thesis.is_some());
    assert!(summary.wait_thesis.as_ref().unwrap().should_wait);
}

#[test]
fn action_scores_always_in_canonical_order() {
    let risk = make_risk(ThesisDirection::Unchanged);
    let pnl = make_pnl(100.0, 100.0);
    let gate = FeatureGateConfig::default();
    let summary = DecisionEngine::synthesize(&risk, &pnl, &SentimentLabel::Neutral, &gate);
    assert_eq!(summary.action_scores.len(), 3);
    assert_eq!(summary.action_scores[0].action, CandidateAction::Sell);
    assert_eq!(summary.action_scores[1].action, CandidateAction::Hold);
    assert_eq!(summary.action_scores[2].action, CandidateAction::BuyMore);
}

#[test]
fn decision_always_has_explanation() {
    let risk = make_risk(ThesisDirection::Unchanged);
    let pnl = make_pnl(100.0, 100.0);
    let gate = FeatureGateConfig::default();
    let summary = DecisionEngine::synthesize(&risk, &pnl, &SentimentLabel::Neutral, &gate);
    assert!(!summary.primary_reasons.is_empty() || summary.recommendation_gated);
}
