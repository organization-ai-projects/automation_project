use crate::plugins::plugin::Plugin;
use crate::plugins::plugin_id::PluginId;

/// Factory for builtin plugin definitions.
pub struct BuiltinPlugins;

impl BuiltinPlugins {
    pub fn log_viewer() -> Plugin {
        Plugin::new(PluginId::new("log_viewer"), "Log Viewer".to_string())
    }

    pub fn report_viewer() -> Plugin {
        Plugin::new(PluginId::new("report_viewer"), "Report Viewer".to_string())
    }

    pub fn graph_viewer() -> Plugin {
        Plugin::new(PluginId::new("graph_viewer"), "Graph Viewer".to_string())
    }

    /// Returns all builtin plugins in deterministic (sorted) order.
    pub fn all() -> Vec<Plugin> {
        let mut plugins = vec![
            Self::log_viewer(),
            Self::report_viewer(),
            Self::graph_viewer(),
        ];
        plugins.sort_by(|a, b| a.id().cmp(b.id()));
        plugins
    }
}
