use crate::decision::DecisionSummary;
use crate::report::DecisionReport;

#[test]
fn new_creates_report_with_hash() {
    let summary = DecisionSummary::gated();
    let report = DecisionReport::new("AAPL", "2025-01-15T10:00:00Z", summary);
    assert_eq!(report.ticker, "AAPL");
    assert!(!report.run_hash.0.is_empty());
}

#[test]
fn deterministic_hash() {
    let s1 = DecisionSummary::gated();
    let s2 = DecisionSummary::gated();
    let r1 = DecisionReport::new("AAPL", "2025-01-15T10:00:00Z", s1);
    let r2 = DecisionReport::new("AAPL", "2025-01-15T10:00:00Z", s2);
    assert_eq!(r1.run_hash, r2.run_hash);
}

#[test]
fn serialization_roundtrip() {
    let summary = DecisionSummary::gated();
    let report = DecisionReport::new("AAPL", "2025-01-15T10:00:00Z", summary);
    let json = common_json::to_json_string(&report).unwrap();
    let restored: DecisionReport = common_json::from_str(&json).unwrap();
    assert_eq!(report, restored);
}
