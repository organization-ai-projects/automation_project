// projects/products/unstable/hospital_tycoon/ui/src/transport/ipc_client.rs
use crate::diagnostics::error::AppError;
use crate::transport::backend_process::BackendProcess;
use common_json::JsonAccess;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotDto {
    pub tick: u64,
    pub patient_count: usize,
    pub budget_balance: i64,
    pub reputation_score: u32,
    pub patients_treated: u32,
    pub hash: String,
    pub data_summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunReportDto {
    pub seed: u64,
    pub scenario_name: String,
    pub total_ticks: u64,
    pub patients_treated: u32,
    pub patients_died: u32,
    pub final_budget: i64,
    pub final_reputation: u32,
    pub event_count: usize,
    pub run_hash: String,
}

pub struct IpcClient {
    next_id: u64,
    stdin: std::process::ChildStdin,
    reader: BufReader<std::process::ChildStdout>,
    _process: BackendProcess,
}

#[derive(Debug, Clone, Serialize)]
struct OutboundMessage {
    id: u64,
    request: OutboundRequest,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
enum OutboundRequest {
    NewRun { seed: u64, ticks: u64 },
    RunToEnd,
    GetReport,
    SaveReplay { path: String },
    LoadReplay { path: String },
    ReplayToEnd,
}

impl IpcClient {
    pub fn new(mut process: BackendProcess) -> Self {
        let stdin = process.child.stdin.take().expect("stdin");
        let stdout = process.child.stdout.take().expect("stdout");
        Self {
            next_id: 1,
            stdin,
            reader: BufReader::new(stdout),
            _process: process,
        }
    }

    fn send(&mut self, request: OutboundRequest) -> Result<common_json::Json, AppError> {
        let id = self.next_id;
        self.next_id += 1;
        let msg = OutboundMessage { id, request };
        let line = common_json::to_string(&msg).map_err(|e| AppError::Ipc(e.to_string()))?;
        writeln!(self.stdin, "{}", line).map_err(|e| AppError::Ipc(e.to_string()))?;
        self.stdin
            .flush()
            .map_err(|e| AppError::Ipc(e.to_string()))?;
        let mut resp_line = String::new();
        self.reader
            .read_line(&mut resp_line)
            .map_err(|e| AppError::Ipc(e.to_string()))?;
        common_json::from_str(&resp_line).map_err(|e| AppError::Ipc(e.to_string()))
    }

    pub fn new_run(&mut self, seed: u64, ticks: u64) -> Result<(), AppError> {
        let req = OutboundRequest::NewRun { seed, ticks };
        self.send(req)?;
        Ok(())
    }

    pub fn run_to_end(&mut self) -> Result<SnapshotDto, AppError> {
        let req = OutboundRequest::RunToEnd;
        let resp = self.send(req)?;
        let response_type = resp
            .get_field("type")
            .ok()
            .and_then(common_json::Json::as_str)
            .unwrap_or_default();
        if response_type == "Snapshot" {
            let snapshot_json = resp
                .get_field("snapshot")
                .map_err(|e| AppError::Ipc(e.to_string()))?;
            let snap: SnapshotDto = common_json::from_value(snapshot_json.clone())
                .map_err(|e| AppError::Ipc(e.to_string()))?;
            Ok(snap)
        } else {
            Err(AppError::Ipc(format!("unexpected response: {:?}", resp)))
        }
    }

    pub fn get_report(&mut self) -> Result<RunReportDto, AppError> {
        let req = OutboundRequest::GetReport;
        let resp = self.send(req)?;
        let response_type = resp
            .get_field("type")
            .ok()
            .and_then(common_json::Json::as_str)
            .unwrap_or_default();
        if response_type == "Report" {
            let report_json = resp
                .get_field("report")
                .map_err(|e| AppError::Ipc(e.to_string()))?;
            let report: RunReportDto = common_json::from_value(report_json.clone())
                .map_err(|e| AppError::Ipc(e.to_string()))?;
            Ok(report)
        } else {
            Err(AppError::Ipc(format!("unexpected response: {:?}", resp)))
        }
    }

    pub fn save_replay(&mut self, path: &str) -> Result<(), AppError> {
        let req = OutboundRequest::SaveReplay {
            path: path.to_string(),
        };
        self.send(req)?;
        Ok(())
    }

    pub fn load_replay(&mut self, path: &str) -> Result<(), AppError> {
        let req = OutboundRequest::LoadReplay {
            path: path.to_string(),
        };
        self.send(req)?;
        Ok(())
    }

    pub fn replay_to_end(&mut self) -> Result<RunReportDto, AppError> {
        let req = OutboundRequest::ReplayToEnd;
        let resp = self.send(req)?;
        let response_type = resp
            .get_field("type")
            .ok()
            .and_then(common_json::Json::as_str)
            .unwrap_or_default();
        if response_type == "Report" {
            let report_json = resp
                .get_field("report")
                .map_err(|e| AppError::Ipc(e.to_string()))?;
            let report: RunReportDto = common_json::from_value(report_json.clone())
                .map_err(|e| AppError::Ipc(e.to_string()))?;
            Ok(report)
        } else {
            Err(AppError::Ipc(format!("unexpected response: {:?}", resp)))
        }
    }

    pub fn shutdown(self) {}
}
