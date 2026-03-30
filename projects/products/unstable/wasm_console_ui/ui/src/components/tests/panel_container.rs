use crate::components::panel_container::PanelContainer;

#[test]
fn panel_container_empty() {
    let container = PanelContainer::new(None, vec![]);
    assert!(!container.has_active_panel());
    assert_eq!(container.panel_count(), 0);
}

#[test]
fn panel_container_with_panels() {
    let container = PanelContainer::new(
        Some("log_viewer".to_string()),
        vec!["Log Viewer".to_string(), "Report Viewer".to_string()],
    );
    assert!(container.has_active_panel());
    assert_eq!(container.panel_count(), 2);
}
