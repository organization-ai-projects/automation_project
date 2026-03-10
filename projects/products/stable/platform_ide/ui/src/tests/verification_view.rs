use crate::finding_entry::FindingEntry;
use crate::verification_view::VerificationView;

#[test]
fn load_sets_results() {
    let mut view = VerificationView::default();
    view.load(
        false,
        vec![FindingEntry {
            severity: "error".to_string(),
            summary: "undefined variable".to_string(),
            path: Some("src/main.rs".to_string()),
            line: Some(10),
        }],
    );
    assert!(!view.healthy);
    assert_eq!(view.findings.len(), 1);
    assert!(!view.running);
}

#[test]
fn healthy_result_is_empty() {
    let mut view = VerificationView::default();
    view.load(true, vec![]);
    assert!(view.healthy);
    assert!(view.findings.is_empty());
}
