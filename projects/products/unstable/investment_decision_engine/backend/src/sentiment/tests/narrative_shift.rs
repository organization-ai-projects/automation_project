use crate::sentiment::narrative_shift::{NarrativeDirection, NarrativeShift};

#[test]
fn bearish_shift_detected() {
    let shift = NarrativeShift::new(
        "2025-01-15",
        NarrativeDirection::BullishToBearish,
        "Market panic",
        0.85,
    );
    assert!(shift.is_bearish_shift());
}

#[test]
fn bullish_shift_not_bearish() {
    let shift = NarrativeShift::new(
        "2025-01-15",
        NarrativeDirection::BearishToBullish,
        "Recovery",
        0.7,
    );
    assert!(!shift.is_bearish_shift());
}

#[test]
fn serialization_roundtrip() {
    let shift = NarrativeShift::new("2025-01-15", NarrativeDirection::Unchanged, "Stable", 0.9);
    let json = common_json::to_json_string(&shift).unwrap();
    let restored: NarrativeShift = common_json::from_str(&json).unwrap();
    assert_eq!(shift, restored);
}
