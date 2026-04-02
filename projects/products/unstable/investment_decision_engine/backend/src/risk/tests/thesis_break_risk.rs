use crate::history::thesis_change::ThesisDirection;
use crate::risk::ThesisBreakRisk;

#[test]
fn broken_thesis_max_risk() {
    let risk = ThesisBreakRisk::compute(ThesisDirection::Broken);
    assert!((risk.score - 1.0).abs() < f64::EPSILON);
}

#[test]
fn strengthened_thesis_zero_risk() {
    let risk = ThesisBreakRisk::compute(ThesisDirection::Strengthened);
    assert!((risk.score - 0.0).abs() < f64::EPSILON);
}
