use crate::assets::AssetId;
use crate::portfolio::{CostBasis, Position};

#[test]
fn total_cost_is_shares_times_average_price() {
    let pos = Position::new(
        AssetId::new("AAPL"),
        10.0,
        CostBasis::new(150.0, 1500.0),
    );
    assert!((pos.total_cost() - 1500.0).abs() < f64::EPSILON);
}

#[test]
fn serialization_roundtrip() {
    let pos = Position::new(
        AssetId::new("GOOG"),
        5.0,
        CostBasis::from_single_purchase(2800.0, 5.0),
    );
    let json = common_json::to_json_string(&pos).unwrap();
    let restored: Position = common_json::from_str(&json).unwrap();
    assert_eq!(pos, restored);
}
