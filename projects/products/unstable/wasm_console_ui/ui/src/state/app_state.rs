use serde::{Deserialize, Serialize};

/// UI-side application state, derived from backend responses.
/// This is NOT the authoritative state - the backend is.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppState {
    pub active_panel_id: Option<String>,
    pub panel_titles: Vec<String>,
    pub status_message: Option<String>,
    pub error_message: Option<String>,
    pub log_content: Option<String>,
    pub report_content: Option<String>,
    pub graph_content: Option<String>,
    pub snapshot_json: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            active_panel_id: None,
            panel_titles: Vec::new(),
            status_message: None,
            error_message: None,
            log_content: None,
            report_content: None,
            graph_content: None,
            snapshot_json: None,
        }
    }

    pub fn has_error(&self) -> bool {
        self.error_message.is_some()
    }

    pub fn has_status(&self) -> bool {
        self.status_message.is_some()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
