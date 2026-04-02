use crate::portfolio::CostBasis;

#[test]
fn new_stores_values() {
    let cb = CostBasis::new(100.0, 500.0);
    assert!((cb.average_price - 100.0).abs() < f64::EPSILON);
    assert!((cb.total_invested - 500.0).abs() < f64::EPSILON);
}

#[test]
fn from_single_purchase_computes_total() {
    let cb = CostBasis::from_single_purchase(50.0, 10.0);
    assert!((cb.average_price - 50.0).abs() < f64::EPSILON);
    assert!((cb.total_invested - 500.0).abs() < f64::EPSILON);
}
