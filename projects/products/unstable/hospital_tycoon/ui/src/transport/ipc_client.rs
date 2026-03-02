// projects/products/unstable/hospital_tycoon/ui/src/transport/ipc_client.rs
use crate::diagnostics::error::AppError;
use crate::transport::backend_process::BackendProcess;
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

    fn send(&mut self, request: serde_json::Value) -> Result<serde_json::Value, AppError> {
        let id = self.next_id;
        self.next_id += 1;
        let msg = serde_json::json!({ "id": id, "request": request });
        let line = serde_json::to_string(&msg).map_err(|e| AppError::Ipc(e.to_string()))?;
        writeln!(self.stdin, "{}", line).map_err(|e| AppError::Ipc(e.to_string()))?;
        self.stdin
            .flush()
            .map_err(|e| AppError::Ipc(e.to_string()))?;
        let mut resp_line = String::new();
        self.reader
            .read_line(&mut resp_line)
            .map_err(|e| AppError::Ipc(e.to_string()))?;
        serde_json::from_str(&resp_line).map_err(|e| AppError::Ipc(e.to_string()))
    }

    pub fn new_run(&mut self, seed: u64, ticks: u64) -> Result<(), AppError> {
        let req = serde_json::json!({ "type": "NewRun", "seed": seed, "ticks": ticks });
        self.send(req)?;
        Ok(())
    }

    pub fn run_to_end(&mut self) -> Result<SnapshotDto, AppError> {
        let req = serde_json::json!({ "type": "RunToEnd" });
        let resp = self.send(req)?;
        if resp["type"] == "Snapshot" {
            let snap: SnapshotDto = serde_json::from_value(resp["snapshot"].clone())
                .map_err(|e| AppError::Ipc(e.to_string()))?;
            Ok(snap)
        } else {
            Err(AppError::Ipc(format!("unexpected response: {}", resp)))
        }
    }

    pub fn get_report(&mut self) -> Result<RunReportDto, AppError> {
        let req = serde_json::json!({ "type": "GetReport" });
        let resp = self.send(req)?;
        if resp["type"] == "Report" {
            let report: RunReportDto = serde_json::from_value(resp["report"].clone())
                .map_err(|e| AppError::Ipc(e.to_string()))?;
            Ok(report)
        } else {
            Err(AppError::Ipc(format!("unexpected response: {}", resp)))
        }
    }

    pub fn save_replay(&mut self, path: &str) -> Result<(), AppError> {
        let req = serde_json::json!({ "type": "SaveReplay", "path": path });
        self.send(req)?;
        Ok(())
    }

    pub fn load_replay(&mut self, path: &str) -> Result<(), AppError> {
        let req = serde_json::json!({ "type": "LoadReplay", "path": path });
        self.send(req)?;
        Ok(())
    }

    pub fn replay_to_end(&mut self) -> Result<RunReportDto, AppError> {
        let req = serde_json::json!({ "type": "ReplayToEnd" });
        let resp = self.send(req)?;
        if resp["type"] == "Report" {
            let report: RunReportDto = serde_json::from_value(resp["report"].clone())
                .map_err(|e| AppError::Ipc(e.to_string()))?;
            Ok(report)
        } else {
            Err(AppError::Ipc(format!("unexpected response: {}", resp)))
        }
    }

    pub fn shutdown(self) {}
}
