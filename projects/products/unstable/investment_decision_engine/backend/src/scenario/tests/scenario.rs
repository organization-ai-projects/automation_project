use crate::scenario::StressCase;
use crate::scenario::scenario::Scenario;

#[test]
fn new_creates_empty_scenario() {
    let s = Scenario::new("Bear case", "Market crash");
    assert_eq!(s.name, "Bear case");
    assert!(s.stress_cases.is_empty());
}

#[test]
fn add_stress_case_appends() {
    let mut s = Scenario::new("Test", "Test scenario");
    s.add_stress_case(StressCase::new("Crash", -0.5, -0.3, 0.2));
    assert_eq!(s.stress_cases.len(), 1);
}

#[test]
fn serialization_roundtrip() {
    let mut s = Scenario::new("Test", "Description");
    s.add_stress_case(StressCase::new("Case1", -0.2, -0.1, 0.3));
    let json = common_json::to_json_string(&s).unwrap();
    let restored: Scenario = common_json::from_str(&json).unwrap();
    assert_eq!(s, restored);
}
