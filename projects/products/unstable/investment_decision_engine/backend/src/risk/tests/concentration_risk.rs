use crate::risk::ConcentrationRisk;

#[test]
fn low_weight_low_risk() {
    let risk = ConcentrationRisk::compute(0.05);
    assert!((risk.score - 0.1).abs() < f64::EPSILON);
}

#[test]
fn high_weight_high_risk() {
    let risk = ConcentrationRisk::compute(0.6);
    assert!((risk.score - 1.0).abs() < f64::EPSILON);
}
