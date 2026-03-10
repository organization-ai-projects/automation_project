use crate::offline_controls::OfflineControls;

#[test]
fn hidden_by_default() {
    let controls = OfflineControls::default();
    assert!(!controls.visible);
}

#[test]
fn show_makes_visible() {
    let mut controls = OfflineControls::default();
    controls.show(Some("Admin approved.".to_string()));
    assert!(controls.visible);
    assert_eq!(controls.notice.as_deref(), Some("Admin approved."));
}

#[test]
fn hide_clears_state() {
    let mut controls = OfflineControls::default();
    controls.show(Some("notice".to_string()));
    controls.hide();
    assert!(!controls.visible);
    assert!(controls.notice.is_none());
}
