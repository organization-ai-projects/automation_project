use crate::history::company_event::{CompanyEvent, EventCategory};
use crate::history::CompanyTimeline;

#[test]
fn add_event_appends() {
    let mut timeline = CompanyTimeline::new("AAPL");
    timeline.add_event(CompanyEvent::new("2025-01-15", EventCategory::Earnings, "Q4", "Beat"));
    assert_eq!(timeline.events.len(), 1);
}

#[test]
fn events_in_range_filters_correctly() {
    let mut timeline = CompanyTimeline::new("AAPL");
    timeline.add_event(CompanyEvent::new("2025-01-10", EventCategory::Earnings, "Q4", "Beat"));
    timeline.add_event(CompanyEvent::new("2025-01-20", EventCategory::ProductLaunch, "New iPhone", "Released"));
    timeline.add_event(CompanyEvent::new("2025-02-01", EventCategory::GuidanceChange, "Guidance", "Lowered"));

    let filtered = timeline.events_in_range("2025-01-01", "2025-01-31");
    assert_eq!(filtered.len(), 2);
}
