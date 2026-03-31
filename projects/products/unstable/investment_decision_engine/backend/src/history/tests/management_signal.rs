use crate::history::management_signal::{ManagementSignal, SignalDirection};

#[test]
fn new_creates_signal() {
    let signal = ManagementSignal::new("2025-01-15", SignalDirection::Positive, "CEO Interview", "Optimistic outlook");
    assert_eq!(signal.direction, SignalDirection::Positive);
}

#[test]
fn serialization_roundtrip() {
    let signal = ManagementSignal::new("2025-01-15", SignalDirection::Negative, "CFO Call", "Cost concerns");
    let json = common_json::to_json_string(&signal).unwrap();
    let restored: ManagementSignal = common_json::from_str(&json).unwrap();
    assert_eq!(signal, restored);
}
