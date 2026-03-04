use crate::diagnostics::ui_error::UiError;

use super::backend_process::BackendProcess;

mod wire {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize)]
    pub struct Request {
        pub id: Option<String>,
        #[serde(flatten)]
        pub payload: RequestPayload,
    }

    #[derive(Debug, Clone, Serialize)]
    #[serde(tag = "type", rename_all = "camelCase")]
    pub enum RequestPayload {
        LoadModel { model: String },
        ValidateModel,
        NewRun { seed: u64 },
        Step,
        RunToEnd,
        GetSnapshot,
        GetReport,
        SaveReplay,
        GetReplay,
        LoadReplay { replay: String },
        ReplayToEnd,
        Shutdown,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct Response {
        pub id: Option<String>,
        #[serde(flatten)]
        pub payload: ResponsePayload,
    }

    #[derive(Debug, Clone, Deserialize)]
    #[serde(tag = "type", rename_all = "camelCase")]
    pub enum ResponsePayload {
        Ok,
        Error {
            message: String,
        },
        Snapshot {
            hash: String,
            state_json: String,
        },
        Report {
            run_hash: String,
            report_json: String,
        },
        ReplayData {
            replay: String,
        },
    }
}

pub struct IpcClient {
    backend: Option<BackendProcess>,
    request_index: u64,
}

impl IpcClient {
    pub fn new(process: BackendProcess) -> Self {
        Self {
            backend: Some(process),
            request_index: 0,
        }
    }

    pub fn send_load_model(&mut self, model: String) -> Result<(), UiError> {
        Self::expect_ok(self.request(wire::RequestPayload::LoadModel { model })?)
    }

    pub fn send_validate_model(&mut self) -> Result<(), UiError> {
        Self::expect_ok(self.request(wire::RequestPayload::ValidateModel)?)
    }

    pub fn send_new_run(&mut self, seed: u64) -> Result<(), UiError> {
        Self::expect_ok(self.request(wire::RequestPayload::NewRun { seed })?)
    }

    pub fn send_step(&mut self) -> Result<(), UiError> {
        Self::expect_ok(self.request(wire::RequestPayload::Step)?)
    }

    pub fn send_run_to_end(&mut self) -> Result<(), UiError> {
        Self::expect_ok(self.request(wire::RequestPayload::RunToEnd)?)
    }

    pub fn send_get_snapshot(&mut self) -> Result<Option<(String, String)>, UiError> {
        match self.request(wire::RequestPayload::GetSnapshot)? {
            wire::ResponsePayload::Snapshot { hash, state_json } => Ok(Some((hash, state_json))),
            wire::ResponsePayload::Ok => Ok(None),
            wire::ResponsePayload::Error { message } => Err(UiError::Ipc(message)),
            wire::ResponsePayload::Report { .. } => Err(UiError::Ipc(
                "unexpected report response while requesting snapshot".to_string(),
            )),
            wire::ResponsePayload::ReplayData { .. } => Err(UiError::Ipc(
                "unexpected replay response while requesting snapshot".to_string(),
            )),
        }
    }

    pub fn send_get_report(&mut self) -> Result<Option<(String, String)>, UiError> {
        match self.request(wire::RequestPayload::GetReport)? {
            wire::ResponsePayload::Report {
                run_hash,
                report_json,
            } => Ok(Some((run_hash, report_json))),
            wire::ResponsePayload::Ok => Ok(None),
            wire::ResponsePayload::Error { message } => Err(UiError::Ipc(message)),
            wire::ResponsePayload::Snapshot { .. } => Err(UiError::Ipc(
                "unexpected snapshot response while requesting report".to_string(),
            )),
            wire::ResponsePayload::ReplayData { .. } => Err(UiError::Ipc(
                "unexpected replay response while requesting report".to_string(),
            )),
        }
    }

    pub fn send_save_replay(&mut self) -> Result<(), UiError> {
        Self::expect_ok(self.request(wire::RequestPayload::SaveReplay)?)
    }

    pub fn send_get_replay(&mut self) -> Result<Option<String>, UiError> {
        match self.request(wire::RequestPayload::GetReplay)? {
            wire::ResponsePayload::ReplayData { replay } => Ok(Some(replay)),
            wire::ResponsePayload::Ok => Ok(None),
            wire::ResponsePayload::Error { message } => Err(UiError::Ipc(message)),
            wire::ResponsePayload::Snapshot { .. } | wire::ResponsePayload::Report { .. } => Err(
                UiError::Ipc("unexpected response while requesting replay".to_string()),
            ),
        }
    }

    pub fn send_load_replay(&mut self, replay: String) -> Result<(), UiError> {
        Self::expect_ok(self.request(wire::RequestPayload::LoadReplay { replay })?)
    }

    pub fn send_replay_to_end(&mut self) -> Result<(), UiError> {
        Self::expect_ok(self.request(wire::RequestPayload::ReplayToEnd)?)
    }

    pub fn close(&mut self) {
        let _ = self.request(wire::RequestPayload::Shutdown);
        if let Some(process) = self.backend.take() {
            process.shutdown();
        }
    }

    fn request(&mut self, payload: wire::RequestPayload) -> Result<wire::ResponsePayload, UiError> {
        let backend = self
            .backend
            .as_mut()
            .ok_or_else(|| UiError::State("backend process already closed".to_string()))?;
        self.request_index += 1;
        let request = wire::Request {
            id: Some(format!("ui-{}", self.request_index)),
            payload,
        };
        let line =
            common_json::to_string(&request).map_err(|error| UiError::Ipc(error.to_string()))?;
        backend.send_line(&line)?;

        let response_line = backend.read_line()?;
        let response: wire::Response = common_json::from_json_str(response_line.trim())
            .map_err(|error| UiError::Ipc(error.to_string()))?;
        let _response_id = response.id;
        Ok(response.payload)
    }

    fn expect_ok(response: wire::ResponsePayload) -> Result<(), UiError> {
        match response {
            wire::ResponsePayload::Ok => Ok(()),
            wire::ResponsePayload::Error { message } => Err(UiError::Ipc(message)),
            wire::ResponsePayload::Snapshot { .. }
            | wire::ResponsePayload::Report { .. }
            | wire::ResponsePayload::ReplayData { .. } => {
                Err(UiError::Ipc("unexpected non-ok response".to_string()))
            }
        }
    }
}
