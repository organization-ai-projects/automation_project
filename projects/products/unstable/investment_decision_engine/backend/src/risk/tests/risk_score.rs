use crate::history::thesis_change::ThesisDirection;
use crate::risk::{ConcentrationRisk, DrawdownRisk, RiskScore, ThesisBreakRisk, ValuationRisk};

#[test]
fn all_low_risk_is_not_high_risk() {
    let dd = DrawdownRisk::compute(0.0, Some(0.0));
    let cc = ConcentrationRisk::compute(0.05);
    let vr = ValuationRisk::compute(Some(10.0), Some(0.08));
    let tb = ThesisBreakRisk::compute(ThesisDirection::Strengthened);
    let score = RiskScore::compute(dd, cc, vr, tb);
    assert!(!score.is_high_risk());
}

#[test]
fn all_high_risk_is_high_risk() {
    let dd = DrawdownRisk::compute(-0.8, Some(-0.9));
    let cc = ConcentrationRisk::compute(0.6);
    let vr = ValuationRisk::compute(Some(60.0), Some(0.005));
    let tb = ThesisBreakRisk::compute(ThesisDirection::Broken);
    let score = RiskScore::compute(dd, cc, vr, tb);
    assert!(score.is_high_risk());
}

#[test]
fn serialization_roundtrip() {
    let dd = DrawdownRisk::compute(-0.1, None);
    let cc = ConcentrationRisk::compute(0.2);
    let vr = ValuationRisk::compute(Some(20.0), Some(0.04));
    let tb = ThesisBreakRisk::compute(ThesisDirection::Unchanged);
    let score = RiskScore::compute(dd, cc, vr, tb);
    let json = common_json::to_json_string(&score).unwrap();
    let restored: RiskScore = common_json::from_str(&json).unwrap();
    assert_eq!(score, restored);
}
