use crate::neural::neural_summary::{DeclineClassification, NeuralSummary};

#[test]
fn empty_summary_has_no_classifications() {
    let summary = NeuralSummary::empty("AAPL");
    assert!(summary.decline_classification.is_none());
    assert!(summary.company_summary.is_none());
}

#[test]
fn serialization_roundtrip() {
    let mut summary = NeuralSummary::empty("AAPL");
    summary.decline_classification = Some(DeclineClassification::PanicDriven);
    let json = common_json::to_json_string(&summary).unwrap();
    let restored: NeuralSummary = common_json::from_str(&json).unwrap();
    assert_eq!(summary, restored);
}
