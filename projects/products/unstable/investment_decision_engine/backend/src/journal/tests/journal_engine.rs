use crate::decision::DecisionSummary;
use crate::journal::JournalEngine;

#[test]
fn record_gated_decision() {
    let summary = DecisionSummary::gated();
    let entry = JournalEngine::record_decision("AAPL", "2025-01-15T10:00:00Z", &summary);
    assert_eq!(entry.ticker, "AAPL");
}

#[test]
fn create_thesis_snapshot() {
    let snap = JournalEngine::create_thesis_snapshot(
        "AAPL",
        "2025-01-15",
        "Growth thesis",
        vec!["Assumption 1".to_string()],
        vec!["Trigger 1".to_string()],
    );
    assert_eq!(snap.key_assumptions.len(), 1);
    assert_eq!(snap.invalidation_triggers.len(), 1);
}
