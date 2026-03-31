use crate::assets::AssetId;
use crate::portfolio::{CostBasis, PortfolioState, Position};

#[test]
fn total_value_includes_cash_and_positions() {
    let positions = vec![Position::new(
        AssetId::new("AAPL"),
        10.0,
        CostBasis::new(150.0, 1500.0),
    )];
    let state = PortfolioState::new(positions, 500.0);
    assert!((state.total_value - 2000.0).abs() < f64::EPSILON);
}

#[test]
fn position_for_finds_by_ticker() {
    let positions = vec![
        Position::new(AssetId::new("AAPL"), 10.0, CostBasis::new(150.0, 1500.0)),
        Position::new(AssetId::new("GOOG"), 5.0, CostBasis::new(2800.0, 14000.0)),
    ];
    let state = PortfolioState::new(positions, 0.0);
    assert!(state.position_for("GOOG").is_some());
    assert!(state.position_for("MSFT").is_none());
}

#[test]
fn concentration_computes_ratio() {
    let positions = vec![
        Position::new(AssetId::new("AAPL"), 10.0, CostBasis::new(100.0, 1000.0)),
        Position::new(AssetId::new("GOOG"), 10.0, CostBasis::new(100.0, 1000.0)),
    ];
    let state = PortfolioState::new(positions, 0.0);
    assert!((state.concentration("AAPL") - 0.5).abs() < f64::EPSILON);
}

#[test]
fn serialization_roundtrip() {
    let positions = vec![Position::new(
        AssetId::new("TSLA"),
        3.0,
        CostBasis::new(200.0, 600.0),
    )];
    let state = PortfolioState::new(positions, 1000.0);
    let json = common_json::to_json_string(&state).unwrap();
    let restored: PortfolioState = common_json::from_str(&json).unwrap();
    assert_eq!(state, restored);
}
