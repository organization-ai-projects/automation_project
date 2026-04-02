use crate::screens::log_viewer_screen::LogViewerScreen;

#[test]
fn log_viewer_no_content() {
    let screen = LogViewerScreen::new(None);
    assert!(!screen.has_content());
    assert_eq!(screen.line_count(), 0);
}

#[test]
fn log_viewer_with_content() {
    let screen = LogViewerScreen::new(Some("line1\nline2\nline3".to_string()));
    assert!(screen.has_content());
    assert_eq!(screen.line_count(), 3);
}

#[test]
fn log_viewer_title() {
    let screen = LogViewerScreen::new(None);
    assert_eq!(screen.title, "Log Viewer");
}
