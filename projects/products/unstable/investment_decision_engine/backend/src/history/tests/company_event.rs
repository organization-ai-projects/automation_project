use crate::history::company_event::{CompanyEvent, EventCategory};

#[test]
fn new_creates_event() {
    let event = CompanyEvent::new(
        "2025-01-15",
        EventCategory::Earnings,
        "Q4 Earnings",
        "Beat estimates",
    );
    assert_eq!(event.date, "2025-01-15");
    assert_eq!(event.category, EventCategory::Earnings);
    assert!(event.impact_assessment.is_none());
}

#[test]
fn serialization_roundtrip() {
    let event = CompanyEvent::new(
        "2025-01-15",
        EventCategory::Acquisition,
        "Merged with XYZ",
        "Strategic expansion",
    );
    let json = common_json::to_json_string(&event).unwrap();
    let restored: CompanyEvent = common_json::from_str(&json).unwrap();
    assert_eq!(event, restored);
}
