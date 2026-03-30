use crate::plugins::plugin::Plugin;
use crate::plugins::plugin_id::PluginId;

#[test]
fn plugin_new() {
    let plugin = Plugin::new(PluginId::new("test"), "Test".to_string());
    assert_eq!(plugin.id().as_str(), "test");
    assert_eq!(plugin.name(), "Test");
}

#[test]
fn plugin_clone_is_equal() {
    let plugin = Plugin::new(PluginId::new("test"), "Test".to_string());
    let cloned = plugin.clone();
    assert_eq!(plugin, cloned);
}
