use crate::diagnostics::error::UiError;
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
        LoadMachine { machine: String },
        Validate,
        Run { events: Vec<String> },
        Step { event: String },
        TestExhaustive,
        TestFuzz { seed: u64, steps: u64 },
        GetTranscript,
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
        Error { message: String },
        TestReport { report_json: String },
        Transcript { transcript_json: String },
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

    pub fn send_load_machine(&mut self, machine: String) -> Result<(), UiError> {
        Self::expect_ok(self.request(wire::RequestPayload::LoadMachine { machine })?)
    }

    pub fn send_validate(&mut self) -> Result<(), UiError> {
        Self::expect_ok(self.request(wire::RequestPayload::Validate)?)
    }

    pub fn send_run(&mut self, events: Vec<String>) -> Result<(), UiError> {
        Self::expect_ok(self.request(wire::RequestPayload::Run { events })?)
    }

    pub fn send_step(&mut self, event: String) -> Result<(), UiError> {
        Self::expect_ok(self.request(wire::RequestPayload::Step { event })?)
    }

    pub fn send_test_exhaustive(&mut self) -> Result<Option<String>, UiError> {
        match self.request(wire::RequestPayload::TestExhaustive)? {
            wire::ResponsePayload::TestReport { report_json } => Ok(Some(report_json)),
            wire::ResponsePayload::Ok => Ok(None),
            wire::ResponsePayload::Error { message } => Err(UiError::Ipc(message)),
            wire::ResponsePayload::Transcript { .. } => {
                Err(UiError::Ipc("unexpected transcript response".to_string()))
            }
        }
    }

    pub fn send_test_fuzz(&mut self, seed: u64, steps: u64) -> Result<Option<String>, UiError> {
        match self.request(wire::RequestPayload::TestFuzz { seed, steps })? {
            wire::ResponsePayload::TestReport { report_json } => Ok(Some(report_json)),
            wire::ResponsePayload::Ok => Ok(None),
            wire::ResponsePayload::Error { message } => Err(UiError::Ipc(message)),
            wire::ResponsePayload::Transcript { .. } => {
                Err(UiError::Ipc("unexpected transcript response".to_string()))
            }
        }
    }

    pub fn send_get_transcript(&mut self) -> Result<Option<String>, UiError> {
        match self.request(wire::RequestPayload::GetTranscript)? {
            wire::ResponsePayload::Transcript { transcript_json } => Ok(Some(transcript_json)),
            wire::ResponsePayload::Ok => Ok(None),
            wire::ResponsePayload::Error { message } => Err(UiError::Ipc(message)),
            wire::ResponsePayload::TestReport { .. } => {
                Err(UiError::Ipc("unexpected test report response".to_string()))
            }
        }
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
            common_json::to_string(&request).map_err(|e| UiError::Ipc(e.to_string()))?;
        backend.send_line(&line)?;

        let response_line = backend.read_line()?;
        let response: wire::Response = common_json::from_json_str(response_line.trim())
            .map_err(|e| UiError::Ipc(e.to_string()))?;
        Ok(response.payload)
    }

    fn expect_ok(response: wire::ResponsePayload) -> Result<(), UiError> {
        match response {
            wire::ResponsePayload::Ok => Ok(()),
            wire::ResponsePayload::Error { message } => Err(UiError::Ipc(message)),
            wire::ResponsePayload::TestReport { .. }
            | wire::ResponsePayload::Transcript { .. } => {
                Err(UiError::Ipc("unexpected non-ok response".to_string()))
            }
        }
    }
}
