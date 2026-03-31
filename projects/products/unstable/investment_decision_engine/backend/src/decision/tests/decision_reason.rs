use crate::decision::decision_reason::{DecisionReason, ReasonCategory};

#[test]
fn new_creates_reason() {
    let reason = DecisionReason::new(ReasonCategory::Fundamental, "Strong earnings", 0.8);
    assert_eq!(reason.category, ReasonCategory::Fundamental);
    assert!((reason.weight - 0.8).abs() < f64::EPSILON);
}

#[test]
fn serialization_roundtrip() {
    let reason = DecisionReason::new(ReasonCategory::Risk, "High drawdown", 0.6);
    let json = common_json::to_json_string(&reason).unwrap();
    let restored: DecisionReason = common_json::from_str(&json).unwrap();
    assert_eq!(reason, restored);
}
