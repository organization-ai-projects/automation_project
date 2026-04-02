use crate::risk::DrawdownRisk;

#[test]
fn no_drawdown_low_score() {
    let risk = DrawdownRisk::compute(0.0, Some(0.0));
    assert!(risk.score < 0.01);
}

#[test]
fn large_drawdown_high_score() {
    let risk = DrawdownRisk::compute(-0.5, Some(-0.6));
    assert!(risk.score > 0.4);
}

#[test]
fn serialization_roundtrip() {
    let risk = DrawdownRisk::compute(-0.2, Some(-0.15));
    let json = common_json::to_json_string(&risk).unwrap();
    let restored: DrawdownRisk = common_json::from_str(&json).unwrap();
    assert_eq!(risk, restored);
}
