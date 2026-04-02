use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::plugins::plugin_id::PluginId;
use crate::ui_model::panel::Panel;

/// Pure, deterministic reducer for state transitions.
pub struct Reducer;

impl Reducer {
    /// Apply an action to produce a new state. This is a pure function.
    pub fn reduce(state: &AppState, action: &Action) -> AppState {
        let mut next = state.clone();
        match action {
            Action::LoadLogFile { path } => {
                let plugin_id = PluginId::new("log_viewer");
                let panel = Panel::new(plugin_id.clone(), format!("Log: {path}"));
                next.panels.retain(|p| p.plugin_id != plugin_id);
                next.panels.push(panel);
                next.panels.sort_by(|a, b| a.plugin_id.cmp(&b.plugin_id));
                next.active_panel = Some(plugin_id);
                next.status_message = Some(format!("Loaded log file: {path}"));
                next.error_message = None;
            }
            Action::LoadReportFile { path } => {
                let plugin_id = PluginId::new("report_viewer");
                let panel = Panel::new(plugin_id.clone(), format!("Report: {path}"));
                next.panels.retain(|p| p.plugin_id != plugin_id);
                next.panels.push(panel);
                next.panels.sort_by(|a, b| a.plugin_id.cmp(&b.plugin_id));
                next.active_panel = Some(plugin_id);
                next.status_message = Some(format!("Loaded report file: {path}"));
                next.error_message = None;
            }
            Action::LoadGraphFile { path } => {
                let plugin_id = PluginId::new("graph_viewer");
                let panel = Panel::new(plugin_id.clone(), format!("Graph: {path}"));
                next.panels.retain(|p| p.plugin_id != plugin_id);
                next.panels.push(panel);
                next.panels.sort_by(|a, b| a.plugin_id.cmp(&b.plugin_id));
                next.active_panel = Some(plugin_id);
                next.status_message = Some(format!("Loaded graph file: {path}"));
                next.error_message = None;
            }
            Action::SelectPanel { plugin_id } => {
                let id = PluginId::new(plugin_id);
                if next.panels.iter().any(|p| p.plugin_id == id) {
                    next.active_panel = Some(id);
                    next.error_message = None;
                } else {
                    next.error_message = Some(format!("Panel not found: {plugin_id}"));
                }
            }
            Action::ExportSnapshot => {
                next.status_message = Some("Snapshot exported".to_string());
                next.error_message = None;
            }
            Action::ImportSnapshot { data: _ } => {
                // The actual import is handled by SnapshotCodec at a higher level.
                // The reducer just marks that an import was requested.
                next.status_message = Some("Snapshot import requested".to_string());
                next.error_message = None;
            }
            Action::ClearPanelData => {
                next.panels.clear();
                next.active_panel = None;
                next.status_message = Some("Panel data cleared".to_string());
                next.error_message = None;
            }
        }
        next
    }
}
