use crate::plugins::plugin_id::PluginId;

#[test]
fn plugin_id_equality() {
    let a = PluginId::new("test");
    let b = PluginId::new("test");
    assert_eq!(a, b);
}

#[test]
fn plugin_id_ordering() {
    let a = PluginId::new("alpha");
    let b = PluginId::new("beta");
    assert!(a < b);
}

#[test]
fn plugin_id_as_str() {
    let id = PluginId::new("test");
    assert_eq!(id.as_str(), "test");
}

#[test]
fn plugin_id_display() {
    let id = PluginId::new("test");
    assert_eq!(format!("{id}"), "test");
}
