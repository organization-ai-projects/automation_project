use crate::plugins::plugin_id::PluginId;
use crate::ui_model::panel::Panel;

#[test]
fn panel_new_has_no_content() {
    let panel = Panel::new(PluginId::new("test"), "Test".to_string());
    assert!(panel.content.is_none());
}

#[test]
fn panel_with_content() {
    let panel = Panel::new(PluginId::new("test"), "Test".to_string())
        .with_content("hello".to_string());
    assert_eq!(panel.content.as_deref(), Some("hello"));
}

#[test]
fn panel_equality() {
    let a = Panel::new(PluginId::new("test"), "Test".to_string());
    let b = Panel::new(PluginId::new("test"), "Test".to_string());
    assert_eq!(a, b);
}
