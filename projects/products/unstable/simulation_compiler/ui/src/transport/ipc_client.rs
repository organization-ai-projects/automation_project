// projects/products/unstable/simulation_compiler/ui/src/transport/ipc_client.rs
use crate::diagnostics::ui_error::UiError;

pub struct IpcClient;

impl IpcClient {
    pub fn new() -> Self {
        Self
    }

    pub fn send_request(&self, json: &str) -> Result<String, UiError> {
        if json.trim().is_empty() {
            return Err(UiError::Transport("empty IPC request".to_string()));
        }
        if !json.trim_start().starts_with('{') {
            return Err(UiError::Internal(
                "request must be a JSON object".to_string(),
            ));
        }

        tracing::debug!(request = %json, "ipc send (stub)");
        Ok("{\"kind\":\"Report\",\"json\":\"{\\\"success\\\":true}\"}".to_string())
    }
}
