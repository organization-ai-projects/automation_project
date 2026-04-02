use crate::market_data::PricePoint;

#[test]
fn new_creates_price_point() {
    let pp = PricePoint::new("2025-01-15", 100.0, 105.0, 98.0, 103.0);
    assert_eq!(pp.date, "2025-01-15");
    assert!((pp.close - 103.0).abs() < f64::EPSILON);
}

#[test]
fn serialization_roundtrip() {
    let pp = PricePoint::new("2025-01-15", 100.0, 105.0, 98.0, 103.0);
    let json = common_json::to_json_string(&pp).unwrap();
    let restored: PricePoint = common_json::from_str(&json).unwrap();
    assert_eq!(pp, restored);
}
