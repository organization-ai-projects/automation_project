use crate::components::status_banner::StatusBanner;

#[test]
fn format_message() {
    let msg = StatusBanner::format("hello");
    assert_eq!(msg, "[market_tycoon] hello");
}
