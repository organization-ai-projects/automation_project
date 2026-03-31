use serde::{Deserialize, Serialize};

use crate::plugins::plugin_id::PluginId;
use crate::ui_model::panel::Panel;

/// Core application state managed by the reducer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppState {
    pub active_panel: Option<PluginId>,
    pub panels: Vec<Panel>,
    pub status_message: Option<String>,
    pub error_message: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            active_panel: None,
            panels: Vec::new(),
            status_message: None,
            error_message: None,
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
