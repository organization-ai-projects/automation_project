use crate::screens::report_viewer_screen::ReportViewerScreen;

#[test]
fn report_viewer_no_content() {
    let screen = ReportViewerScreen::new(None);
    assert!(!screen.has_content());
}

#[test]
fn report_viewer_with_content() {
    let screen = ReportViewerScreen::new(Some("report data".to_string()));
    assert!(screen.has_content());
}

#[test]
fn report_viewer_title() {
    let screen = ReportViewerScreen::new(None);
    assert_eq!(screen.title, "Report Viewer");
}
