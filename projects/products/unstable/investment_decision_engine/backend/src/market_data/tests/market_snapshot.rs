use crate::market_data::{MarketSnapshot, PriceHistory, PricePoint};

#[test]
fn new_creates_snapshot() {
    let history = PriceHistory::new(
        "AAPL",
        vec![PricePoint::new("2025-01-15", 100.0, 105.0, 98.0, 103.0)],
    );
    let snap = MarketSnapshot::new("AAPL", 103.0, history, "2025-01-15");
    assert_eq!(snap.ticker, "AAPL");
    assert!((snap.current_price - 103.0).abs() < f64::EPSILON);
    assert!(snap.volume_history.is_none());
}

#[test]
fn serialization_roundtrip() {
    let history = PriceHistory::new(
        "AAPL",
        vec![PricePoint::new("2025-01-15", 100.0, 105.0, 98.0, 103.0)],
    );
    let snap = MarketSnapshot::new("AAPL", 103.0, history, "2025-01-15");
    let json = common_json::to_json_string(&snap).unwrap();
    let restored: MarketSnapshot = common_json::from_str(&json).unwrap();
    assert_eq!(snap, restored);
}
