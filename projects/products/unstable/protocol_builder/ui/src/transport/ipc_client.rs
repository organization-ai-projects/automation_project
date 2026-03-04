// projects/products/unstable/protocol_builder/ui/src/transport/ipc_client.rs
use crate::diagnostics::ui_error::UiError;

use super::backend_process::BackendProcess;
use super::message::Message;
use super::payload::Payload;
use super::request::Request;
use super::response::Response;

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

    fn send(&mut self, req: Request) -> Result<Response, UiError> {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);

        let request = Message {
            id,
            payload: Payload::Request(req),
        };
        let line = common_json::to_string(&request)
            .map_err(|e| UiError::IpcError(format!("request serialization failed: {e}")))?;

        let response_line = self.process.exchange(&line)?;
        let response: Message = common_json::from_json_str(&response_line)
            .map_err(|e| UiError::IpcError(format!("response decoding failed: {e}")))?;
        if response.id != id {
            return Err(UiError::IpcError(format!(
                "response id mismatch: expected {id}, got {}",
                response.id
            )));
        }
        match response.payload {
            Payload::Response(payload) => Ok(payload),
            Payload::Request(_) => Err(UiError::IpcError(
                "received request payload from backend".to_string(),
            )),
        }
    }

    pub fn send_load_schema(&mut self, path: &str) -> Result<(), UiError> {
        match self.send(Request::LoadSchema {
            path: path.to_string(),
        })? {
            Response::Ok => Ok(()),
            Response::Error { message } => Err(UiError::IpcError(message)),
            _ => Err(UiError::IpcError("unexpected response".to_string())),
        }
    }

    pub fn send_validate(&mut self) -> Result<(), UiError> {
        match self.send(Request::ValidateSchema)? {
            Response::Ok => Ok(()),
            Response::Error { message } => Err(UiError::IpcError(message)),
            _ => Err(UiError::IpcError("unexpected response".to_string())),
        }
    }

    pub fn send_generate_dry_run(&mut self) -> Result<(String, String), UiError> {
        match self.send(Request::GenerateDryRun)? {
            Response::GenerateReport {
                manifest_hash,
                report_json,
            } => Ok((manifest_hash, report_json)),
            Response::Error { message } => Err(UiError::IpcError(message)),
            _ => Err(UiError::IpcError("unexpected response".to_string())),
        }
    }

    pub fn send_generate_write(&mut self, out_dir: &str) -> Result<(String, String), UiError> {
        match self.send(Request::GenerateWrite {
            out_dir: out_dir.to_string(),
        })? {
            Response::GenerateReport {
                manifest_hash,
                report_json,
            } => Ok((manifest_hash, report_json)),
            Response::Error { message } => Err(UiError::IpcError(message)),
            _ => Err(UiError::IpcError("unexpected response".to_string())),
        }
    }

    pub fn send_shutdown(&mut self) -> Result<(), UiError> {
        let _ = self.send(Request::Shutdown)?;
        Ok(())
    }
}
