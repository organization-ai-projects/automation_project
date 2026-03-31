use super::backend_process::BackendProcess;
use crate::diagnostics::error::UiError;

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
        LoadScenario { scenario: String },
        NewRun { seed: u64 },
        EncounterStep,
        StartEncounter,
        CaptureAttempt,
        StartBattle,
        BattleAction { action: String },
        BattleStep,
        EndBattle,
        GetSnapshot,
        GetReport,
        SaveReplay,
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
        EncounterState {
            encounter_json: String,
        },
        BattleState {
            battle_json: String,
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

    pub fn send_load_scenario(&mut self, scenario: String) -> Result<(), UiError> {
        Self::expect_ok(self.request(wire::RequestPayload::LoadScenario { scenario })?)
    }

    pub fn send_new_run(&mut self, seed: u64) -> Result<(), UiError> {
        Self::expect_ok(self.request(wire::RequestPayload::NewRun { seed })?)
    }

    pub fn send_encounter_step(&mut self) -> Result<String, UiError> {
        match self.request(wire::RequestPayload::EncounterStep)? {
            wire::ResponsePayload::EncounterState { encounter_json } => Ok(encounter_json),
            wire::ResponsePayload::Error { message } => Err(UiError::Ipc(message)),
            _ => Err(UiError::Ipc("unexpected response".to_string())),
        }
    }

    pub fn send_capture_attempt(&mut self) -> Result<String, UiError> {
        match self.request(wire::RequestPayload::CaptureAttempt)? {
            wire::ResponsePayload::EncounterState { encounter_json } => Ok(encounter_json),
            wire::ResponsePayload::Error { message } => Err(UiError::Ipc(message)),
            _ => Err(UiError::Ipc("unexpected response".to_string())),
        }
    }

    pub fn send_start_battle(&mut self) -> Result<String, UiError> {
        match self.request(wire::RequestPayload::StartBattle)? {
            wire::ResponsePayload::BattleState { battle_json } => Ok(battle_json),
            wire::ResponsePayload::Error { message } => Err(UiError::Ipc(message)),
            _ => Err(UiError::Ipc("unexpected response".to_string())),
        }
    }

    pub fn send_battle_action(&mut self, action: String) -> Result<String, UiError> {
        match self.request(wire::RequestPayload::BattleAction { action })? {
            wire::ResponsePayload::BattleState { battle_json } => Ok(battle_json),
            wire::ResponsePayload::Error { message } => Err(UiError::Ipc(message)),
            _ => Err(UiError::Ipc("unexpected response".to_string())),
        }
    }

    pub fn send_battle_step(&mut self) -> Result<String, UiError> {
        match self.request(wire::RequestPayload::BattleStep)? {
            wire::ResponsePayload::BattleState { battle_json } => Ok(battle_json),
            wire::ResponsePayload::Error { message } => Err(UiError::Ipc(message)),
            _ => Err(UiError::Ipc("unexpected response".to_string())),
        }
    }

    pub fn send_end_battle(&mut self) -> Result<(), UiError> {
        Self::expect_ok(self.request(wire::RequestPayload::EndBattle)?)
    }

    pub fn send_get_snapshot(&mut self) -> Result<(String, String), UiError> {
        match self.request(wire::RequestPayload::GetSnapshot)? {
            wire::ResponsePayload::Snapshot { hash, state_json } => Ok((hash, state_json)),
            wire::ResponsePayload::Error { message } => Err(UiError::Ipc(message)),
            _ => Err(UiError::Ipc("unexpected response".to_string())),
        }
    }

    pub fn send_get_report(&mut self) -> Result<(String, String), UiError> {
        match self.request(wire::RequestPayload::GetReport)? {
            wire::ResponsePayload::Report {
                run_hash,
                report_json,
            } => Ok((run_hash, report_json)),
            wire::ResponsePayload::Error { message } => Err(UiError::Ipc(message)),
            _ => Err(UiError::Ipc("unexpected response".to_string())),
        }
    }

    pub fn send_save_replay(&mut self) -> Result<String, UiError> {
        match self.request(wire::RequestPayload::SaveReplay)? {
            wire::ResponsePayload::ReplayData { replay } => Ok(replay),
            wire::ResponsePayload::Error { message } => Err(UiError::Ipc(message)),
            _ => Err(UiError::Ipc("unexpected response".to_string())),
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
        let line = common_json::to_string(&request).map_err(|e| UiError::Ipc(e.to_string()))?;
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
            _ => Err(UiError::Ipc("unexpected non-ok response".to_string())),
        }
    }
}
