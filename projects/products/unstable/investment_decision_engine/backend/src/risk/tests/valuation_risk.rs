use crate::risk::ValuationRisk;

#[test]
fn high_pe_high_risk() {
    let risk = ValuationRisk::compute(Some(60.0), Some(0.005));
    assert!(risk.score > 0.7);
}

#[test]
fn low_pe_low_risk() {
    let risk = ValuationRisk::compute(Some(10.0), Some(0.08));
    assert!(risk.score < 0.2);
}

#[test]
fn none_values_give_moderate_risk() {
    let risk = ValuationRisk::compute(None, None);
    assert!((risk.score - 0.5).abs() < f64::EPSILON);
}
