use crate::scenario::StressCase;

#[test]
fn new_creates_stress_case() {
    let sc = StressCase::new("Bear", -0.4, -0.2, 0.25);
    assert_eq!(sc.label, "Bear");
    assert!((sc.probability - 0.25).abs() < f64::EPSILON);
}

#[test]
fn serialization_roundtrip() {
    let sc = StressCase::new("Bull", 0.3, 0.2, 0.3);
    let json = common_json::to_json_string(&sc).unwrap();
    let restored: StressCase = common_json::from_str(&json).unwrap();
    assert_eq!(sc, restored);
}
