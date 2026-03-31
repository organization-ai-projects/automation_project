use crate::decision::{CandidateAction, DecisionConfidence};
use crate::journal::DecisionEntry;

#[test]
fn new_creates_entry() {
    let entry = DecisionEntry::new(
        "2025-01-15T10:00:00Z",
        "AAPL",
        CandidateAction::Hold,
        DecisionConfidence::from_score(0.75),
        "Thesis intact despite drawdown",
        vec!["Revenue drops > 20%".to_string()],
    );
    assert_eq!(entry.ticker, "AAPL");
    assert_eq!(entry.action, CandidateAction::Hold);
    assert_eq!(entry.invalidation_conditions.len(), 1);
}

#[test]
fn serialization_roundtrip() {
    let entry = DecisionEntry::new(
        "2025-01-15T10:00:00Z",
        "GOOG",
        CandidateAction::BuyMore,
        DecisionConfidence::from_score(0.8),
        "Accumulation opportunity",
        vec![],
    );
    let json = common_json::to_json_string(&entry).unwrap();
    let restored: DecisionEntry = common_json::from_str(&json).unwrap();
    assert_eq!(entry, restored);
}
