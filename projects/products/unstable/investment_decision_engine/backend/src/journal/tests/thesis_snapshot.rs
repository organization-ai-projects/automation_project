use crate::journal::ThesisSnapshot;

#[test]
fn new_creates_snapshot() {
    let snap = ThesisSnapshot::new("AAPL", "2025-01-15", "Growth via services expansion");
    assert_eq!(snap.ticker, "AAPL");
    assert!(snap.key_assumptions.is_empty());
}

#[test]
fn serialization_roundtrip() {
    let mut snap = ThesisSnapshot::new("AAPL", "2025-01-15", "Growth thesis");
    snap.key_assumptions.push("Services revenue grows 15% YoY".to_string());
    let json = common_json::to_json_string(&snap).unwrap();
    let restored: ThesisSnapshot = common_json::from_str(&json).unwrap();
    assert_eq!(snap, restored);
}
