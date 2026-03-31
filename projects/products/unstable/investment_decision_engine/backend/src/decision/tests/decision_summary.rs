use crate::decision::DecisionSummary;

#[test]
fn gated_summary_is_hold_with_zero_confidence() {
    let summary = DecisionSummary::gated();
    assert!(summary.recommendation_gated);
    assert!((summary.confidence.score - 0.0).abs() < f64::EPSILON);
}

#[test]
fn gated_serialization_roundtrip() {
    let summary = DecisionSummary::gated();
    let json = common_json::to_json_string(&summary).unwrap();
    let restored: DecisionSummary = common_json::from_str(&json).unwrap();
    assert_eq!(summary.recommendation_gated, restored.recommendation_gated);
}
