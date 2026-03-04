// projects/products/unstable/simulation_compiler/ui/src/transport/ipc_client.rs
use crate::diagnostics::ui_error::UiError;
use crate::transport::backend_process::BackendProcess;
use serde::{Deserialize, Serialize};

pub struct IpcClient {
    next_id: u64,
}

impl IpcClient {
    pub fn new() -> Self {
        Self { next_id: 1 }
    }

    pub fn send_request(
        &mut self,
        backend: &mut BackendProcess,
        request: CompilerRequest,
    ) -> Result<CompilerResponse, UiError> {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);

        let envelope = IpcMessage {
            id,
            payload: IpcPayload::Request(request),
        };
        let request_json = common_json::to_string(&envelope)
            .map_err(|e| UiError::Internal(format!("failed to serialize IPC request: {e}")))?;

        let response_json = backend.exchange(&request_json)?;
        let response: IpcMessage = common_json::from_json_str(&response_json)
            .map_err(|e| UiError::Transport(format!("invalid IPC response JSON: {e}")))?;

        if response.id != id {
            return Err(UiError::Transport(format!(
                "IPC response id mismatch: expected {id}, got {}",
                response.id
            )));
        }

        match response.payload {
            IpcPayload::Response(payload) => Ok(payload),
            IpcPayload::Request(_) => Err(UiError::Transport(
                "received request payload from backend".to_string(),
            )),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcMessage {
    pub id: u64,
    pub payload: IpcPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "direction")]
pub enum IpcPayload {
    Request(CompilerRequest),
    Response(CompilerResponse),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum CompilerRequest {
    LoadDsl { source: String },
    Validate,
    CompileDryRun,
    CompileWrite { out_dir: String },
    GetReport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum CompilerResponse {
    Ok,
    Report { json: String },
    Error { message: String },
}
