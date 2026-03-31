use crate::decision::WaitThesis;

#[test]
fn hold_recommended_sets_should_wait() {
    let wt = WaitThesis::hold_recommended("Recovery likely", 0.7, Some("6 months".to_string()));
    assert!(wt.should_wait);
    assert!((wt.recovery_probability - 0.7).abs() < f64::EPSILON);
}

#[test]
fn exit_recommended_sets_not_wait() {
    let wt = WaitThesis::exit_recommended("Thesis broken");
    assert!(!wt.should_wait);
    assert!((wt.recovery_probability - 0.0).abs() < f64::EPSILON);
}

#[test]
fn serialization_roundtrip() {
    let wt = WaitThesis::hold_recommended("Recovery likely", 0.65, None);
    let json = common_json::to_json_string(&wt).unwrap();
    let restored: WaitThesis = common_json::from_str(&json).unwrap();
    assert_eq!(wt, restored);
}
