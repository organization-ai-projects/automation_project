use crate::diagnostics::ui_error::UiError;
use crate::transport::backend_process::BackendProcess;
use common_json::Json;

/// Thin IPC client that serializes requests to JSON and deserializes responses.
/// Uses a simple sequential message ID counter.
pub struct IpcClient {
    process: BackendProcess,
    next_id: u64,
}

impl IpcClient {
    pub fn new(process: BackendProcess) -> Self {
        Self {
            process,
            next_id: 1,
        }
    }

    fn next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn send_load_inputs(&mut self, paths: Vec<String>) -> Result<Json, UiError> {
        let id = self.next_id();
        let paths_json =
            common_json::to_string(&paths).map_err(|err| UiError::Transport(err.to_string()))?;
        let request_json =
            format!(r#"{{"id":{id},"request":{{"type":"load_inputs","paths":{paths_json}}}}}"#);
        self.send_recv_json(&request_json)
    }

    pub fn send_analyze(&mut self) -> Result<Json, UiError> {
        let id = self.next_id();
        let request_json = format!(r#"{{"id":{id},"request":{{"type":"analyze"}}}}"#);
        self.send_recv_json(&request_json)
    }

    pub fn send_render_docs(&mut self) -> Result<Json, UiError> {
        let id = self.next_id();
        let request_json = format!(r#"{{"id":{id},"request":{{"type":"render_docs"}}}}"#);
        self.send_recv_json(&request_json)
    }

    pub fn send_build_bundle(&mut self) -> Result<Json, UiError> {
        let id = self.next_id();
        let request_json = format!(r#"{{"id":{id},"request":{{"type":"build_bundle"}}}}"#);
        self.send_recv_json(&request_json)
    }

    pub fn send_get_bundle(&mut self) -> Result<Json, UiError> {
        let id = self.next_id();
        let request_json = format!(r#"{{"id":{id},"request":{{"type":"get_bundle"}}}}"#);
        self.send_recv_json(&request_json)
    }

    fn send_recv_json(&mut self, request_json: &str) -> Result<Json, UiError> {
        self.process.send_line(request_json)?;
        let raw = self.process.recv_line()?;
        let value: Json =
            common_json::from_json_str(&raw).map_err(|err| UiError::Transport(err.to_string()))?;
        let response = extract_response_object(&value).ok_or_else(|| {
            UiError::Transport("invalid IPC payload: missing response object".to_string())
        })?;

        let response_type = response
            .get("type")
            .and_then(Json::as_str)
            .ok_or_else(|| UiError::Transport("invalid IPC payload: missing type".to_string()))?;
        if response_type == "error" {
            let message = response
                .get("message")
                .and_then(Json::as_str)
                .unwrap_or("backend error");
            return Err(UiError::Transport(message.to_string()));
        }

        Ok(value)
    }
}

fn extract_response_object(payload: &Json) -> Option<&std::collections::HashMap<String, Json>> {
    payload
        .as_object()
        .and_then(|root| root.get("response"))
        .and_then(Json::as_object)
}
