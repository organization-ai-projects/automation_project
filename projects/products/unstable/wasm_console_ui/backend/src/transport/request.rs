use serde::{Deserialize, Serialize};

/// Typed request payloads for backend IPC.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Request {
    LoadLogFile { path: String },
    LoadReportFile { path: String },
    LoadGraphFile { path: String },
    DispatchAction { action_json: String },
    ExportSnapshot,
    ImportSnapshot { data: String },
}
