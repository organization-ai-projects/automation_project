use crate::diagnostics::error::UiError;
use crate::transport::backend_process::BackendProcess;
use crate::transport::message::Message;
use crate::transport::request::Request;
use crate::transport::response::Response;
use std::io::{BufRead, Write};

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

    pub fn send(&mut self, request: Request) -> Result<Response, UiError> {
        let message = Message {
            id: self.next_id,
            payload: request,
        };
        self.next_id = self.next_id.saturating_add(1);

        let line = common_json::to_string(&message).map_err(|e| UiError::Json(e.to_string()))?;
        writeln!(self.process.stdin, "{line}").map_err(|e| UiError::Ipc(e.to_string()))?;
        self.process
            .stdin
            .flush()
            .map_err(|e| UiError::Ipc(e.to_string()))?;

        let mut response_line = String::new();
        self.process
            .stdout
            .read_line(&mut response_line)
            .map_err(|e| UiError::Ipc(e.to_string()))?;

        if response_line.trim().is_empty() {
            return Err(UiError::Ipc("empty IPC response".to_string()));
        }

        let response: Message<Response> =
            common_json::from_json_str(&response_line).map_err(|e| UiError::Json(e.to_string()))?;
        Ok(response.payload)
    }
}
