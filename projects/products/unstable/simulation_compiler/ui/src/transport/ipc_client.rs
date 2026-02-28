// projects/products/unstable/simulation_compiler/ui/src/transport/ipc_client.rs
use crate::diagnostics::error::UiError;

pub struct IpcClient;

impl IpcClient {
    pub fn new() -> Self {
        Self
    }

    pub fn send_request(&self, json: &str) -> Result<String, UiError> {
        tracing::debug!(request = %json, "ipc send (stub)");
        Ok("{\"kind\":\"Ok\"}".to_string())
    }
}
