use serde::{Deserialize, Serialize};

/// Actions that can be dispatched to mutate application state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    LoadLogFile { path: String },
    LoadReportFile { path: String },
    LoadGraphFile { path: String },
    SelectPanel { plugin_id: String },
    ExportSnapshot,
    ImportSnapshot { data: String },
    ClearPanelData,
}
