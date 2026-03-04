use crate::diagnostics::ui_error::UiError;
use crate::transport::backend_process::BackendProcess;
use crate::transport::message::Message;
use crate::transport::request::Request;
use crate::transport::response::Response;

pub struct IpcClient {
    process: Option<BackendProcess>,
}

impl IpcClient {
    pub fn new(process: BackendProcess) -> Self {
        Self {
            process: Some(process),
        }
    }

    pub fn call(&mut self, request: Request) -> Result<Response, UiError> {
        let process = self
            .process
            .as_mut()
            .ok_or_else(|| UiError::Ipc("backend already closed".to_string()))?;

        let msg = Message { request };
        let line = common_json::to_string(&msg).map_err(|error| UiError::Ipc(error.to_string()))?;
        process.send_line(&line)?;

        let raw = process.recv_line()?;
        common_json::from_str(raw.trim()).map_err(|error| UiError::Ipc(error.to_string()))
    }

    pub fn close(&mut self) {
        if let Some(process) = self.process.as_mut() {
            let msg = Message {
                request: Request::Shutdown,
            };
            if let Ok(line) = common_json::to_string(&msg) {
                let _ = process.send_line(&line);
                let _ = process.recv_line();
            }
        }
        if let Some(process) = self.process.take() {
            process.shutdown();
        }
    }
}
