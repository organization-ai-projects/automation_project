use serde::{Deserialize, Serialize};

/// Typed response payloads from backend IPC.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Response {
    StateSnapshot { json: String },
    PanelData { plugin_id: String, content: String },
    OperationSuccess { message: String },
    Error { message: String },
}
