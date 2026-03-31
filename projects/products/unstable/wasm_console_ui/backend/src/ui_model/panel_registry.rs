use crate::plugins::plugin::Plugin;
use crate::plugins::plugin_id::PluginId;

/// Deterministic registry of panel plugins.
/// Plugins are stored in sorted order by plugin_id for determinism.
pub struct PanelRegistry {
    plugins: Vec<Plugin>,
}

impl PanelRegistry {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn register(&mut self, plugin: Plugin) {
        if !self.plugins.iter().any(|p| p.id() == plugin.id()) {
            self.plugins.push(plugin);
            self.plugins.sort_by(|a, b| a.id().cmp(b.id()));
        }
    }

    pub fn get(&self, id: &PluginId) -> Option<&Plugin> {
        self.plugins.iter().find(|p| p.id() == id)
    }

    pub fn all(&self) -> &[Plugin] {
        &self.plugins
    }

    pub fn ids(&self) -> Vec<&PluginId> {
        self.plugins.iter().map(|p| p.id()).collect()
    }

    pub fn len(&self) -> usize {
        self.plugins.len()
    }

    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }
}

impl Default for PanelRegistry {
    fn default() -> Self {
        Self::new()
    }
}
