use crate::plugins::builtin_plugins::BuiltinPlugins;
use crate::plugins::plugin::Plugin;
use crate::plugins::plugin_id::PluginId;
use crate::ui_model::panel_registry::PanelRegistry;

#[test]
fn registry_starts_empty() {
    let registry = PanelRegistry::new();
    assert!(registry.is_empty());
    assert_eq!(registry.len(), 0);
}

#[test]
fn register_adds_plugin() {
    let mut registry = PanelRegistry::new();
    registry.register(Plugin::new(PluginId::new("test"), "Test Plugin".to_string()));
    assert_eq!(registry.len(), 1);
}

#[test]
fn register_deduplicates() {
    let mut registry = PanelRegistry::new();
    registry.register(Plugin::new(PluginId::new("test"), "Test Plugin".to_string()));
    registry.register(Plugin::new(PluginId::new("test"), "Test Plugin 2".to_string()));
    assert_eq!(registry.len(), 1);
}

#[test]
fn registry_order_is_deterministic() {
    let mut registry = PanelRegistry::new();
    let builtins = BuiltinPlugins::all();
    for plugin in builtins {
        registry.register(plugin);
    }
    let ids: Vec<&str> = registry.ids().iter().map(|id| id.as_str()).collect();
    assert_eq!(ids, vec!["graph_viewer", "log_viewer", "report_viewer"]);
}

#[test]
fn registry_get_returns_registered_plugin() {
    let mut registry = PanelRegistry::new();
    let id = PluginId::new("test");
    registry.register(Plugin::new(id.clone(), "Test Plugin".to_string()));
    assert!(registry.get(&id).is_some());
}

#[test]
fn registry_get_returns_none_for_missing() {
    let registry = PanelRegistry::new();
    let id = PluginId::new("missing");
    assert!(registry.get(&id).is_none());
}
