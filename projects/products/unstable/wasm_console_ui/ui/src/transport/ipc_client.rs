use serde::{Deserialize, Serialize};

/// Typed request sent from UI to backend.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClientRequest {
    LoadLogFile { path: String },
    LoadReportFile { path: String },
    LoadGraphFile { path: String },
    ExportSnapshot,
    ImportSnapshot { data: String },
}

/// Typed response received by UI from backend.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClientResponse {
    StateSnapshot { json: String },
    PanelData { plugin_id: String, content: String },
    OperationSuccess { message: String },
    Error { message: String },
}

/// IPC client for communicating with the backend.
/// In v0, this serializes requests and deserializes responses as JSON.
pub struct IpcClient {
    last_response: Option<ClientResponse>,
}

impl IpcClient {
    pub fn new() -> Self {
        Self {
            last_response: None,
        }
    }

    /// Serialize a request to JSON for sending to backend.
    pub fn serialize_request(request: &ClientRequest) -> Result<String, String> {
        common_json::to_string(request).map_err(|e| e.to_string())
    }

    /// Deserialize a response from backend JSON.
    pub fn deserialize_response(json: &str) -> Result<ClientResponse, String> {
        common_json::from_str(json).map_err(|e| e.to_string())
    }

    /// Store the last response received from backend.
    pub fn set_last_response(&mut self, response: ClientResponse) {
        self.last_response = Some(response);
    }

    /// Get the last response received from backend.
    pub fn last_response(&self) -> Option<&ClientResponse> {
        self.last_response.as_ref()
    }
}

impl Default for IpcClient {
    fn default() -> Self {
        Self::new()
    }
}
