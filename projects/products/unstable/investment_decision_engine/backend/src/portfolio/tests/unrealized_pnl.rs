use crate::assets::AssetId;
use crate::portfolio::{CostBasis, Position, UnrealizedPnl};

#[test]
fn compute_gain() {
    let pos = Position::new(AssetId::new("AAPL"), 10.0, CostBasis::new(100.0, 1000.0));
    let pnl = UnrealizedPnl::compute(&pos, 120.0);
    assert!(pnl.is_gain());
    assert!(!pnl.is_loss());
    assert!((pnl.unrealized_gain_loss - 200.0).abs() < f64::EPSILON);
    assert!((pnl.unrealized_gain_loss_pct - 0.2).abs() < f64::EPSILON);
}

#[test]
fn compute_loss() {
    let pos = Position::new(AssetId::new("AAPL"), 10.0, CostBasis::new(100.0, 1000.0));
    let pnl = UnrealizedPnl::compute(&pos, 80.0);
    assert!(pnl.is_loss());
    assert!(!pnl.is_gain());
    assert!((pnl.unrealized_gain_loss - (-200.0)).abs() < f64::EPSILON);
    assert!((pnl.drawdown_from_purchase - (-0.2)).abs() < f64::EPSILON);
}

#[test]
fn breakeven_is_neither_gain_nor_loss() {
    let pos = Position::new(AssetId::new("AAPL"), 10.0, CostBasis::new(100.0, 1000.0));
    let pnl = UnrealizedPnl::compute(&pos, 100.0);
    assert!(!pnl.is_gain());
    assert!(!pnl.is_loss());
}

#[test]
fn serialization_roundtrip() {
    let pos = Position::new(AssetId::new("AAPL"), 10.0, CostBasis::new(100.0, 1000.0));
    let pnl = UnrealizedPnl::compute(&pos, 110.0);
    let json = common_json::to_json_string(&pnl).unwrap();
    let restored: UnrealizedPnl = common_json::from_str(&json).unwrap();
    assert_eq!(pnl, restored);
}
