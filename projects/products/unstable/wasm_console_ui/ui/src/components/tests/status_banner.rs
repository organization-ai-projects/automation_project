use crate::components::status_banner::StatusBanner;

#[test]
fn banner_no_messages() {
    let banner = StatusBanner::new(None, None);
    assert!(!banner.has_error());
    assert!(!banner.has_status());
    assert_eq!(banner.display_text(), "");
}

#[test]
fn banner_with_status() {
    let banner = StatusBanner::new(Some("Loading...".to_string()), None);
    assert!(banner.has_status());
    assert!(!banner.has_error());
    assert_eq!(banner.display_text(), "Loading...");
}

#[test]
fn banner_with_error() {
    let banner = StatusBanner::new(None, Some("Failed!".to_string()));
    assert!(banner.has_error());
    assert_eq!(banner.display_text(), "Failed!");
}

#[test]
fn banner_error_takes_priority() {
    let banner = StatusBanner::new(Some("status".to_string()), Some("error".to_string()));
    assert_eq!(banner.display_text(), "error");
}
