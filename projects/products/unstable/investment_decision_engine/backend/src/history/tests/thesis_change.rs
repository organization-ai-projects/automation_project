use crate::history::thesis_change::{ThesisChange, ThesisDirection};

#[test]
fn new_creates_change() {
    let tc = ThesisChange::new(
        "2025-01-15",
        ThesisDirection::Weakened,
        "Revenue miss",
        "Growth thesis",
    );
    assert!(!tc.is_broken());
}

#[test]
fn broken_thesis_detected() {
    let tc = ThesisChange::new(
        "2025-01-15",
        ThesisDirection::Broken,
        "Fraud discovered",
        "Integrity thesis",
    );
    assert!(tc.is_broken());
}

#[test]
fn serialization_roundtrip() {
    let tc = ThesisChange::new(
        "2025-01-15",
        ThesisDirection::Strengthened,
        "Beat estimates",
        "Growth thesis",
    );
    let json = common_json::to_json_string(&tc).unwrap();
    let restored: ThesisChange = common_json::from_str(&json).unwrap();
    assert_eq!(tc, restored);
}
