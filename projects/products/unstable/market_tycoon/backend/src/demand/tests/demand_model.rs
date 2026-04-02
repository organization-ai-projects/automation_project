use crate::demand::demand_model::DemandModel;

#[test]
fn default_model() {
    let m = DemandModel::default();
    assert!((m.elasticity - 1.5).abs() < f64::EPSILON);
    assert_eq!(m.base_price_reference, 1000);
}
