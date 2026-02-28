// projects/products/unstable/digital_pet/ui/src/transport/ipc_client.rs
use crate::diagnostics::error::AppError;
use crate::transport::backend_process::BackendProcess;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetStateDto {
    pub species: String,
    pub evolution_stage: u32,
    pub hp: u32,
    pub max_hp: u32,
    pub hunger: u32,
    pub fatigue: u32,
    pub happiness: u32,
    pub discipline: u32,
    pub sick: bool,
    pub tick: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunReportDto {
    pub seed: u64,
    pub final_species: String,
    pub evolution_stage: u32,
    pub total_ticks: u64,
    pub care_mistakes: usize,
    pub final_happiness: u32,
    pub final_discipline: u32,
    pub final_hp: u32,
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
        self.stdin.flush().map_err(|e| AppError::Ipc(e.to_string()))?;
        let mut resp_line = String::new();
        self.reader.read_line(&mut resp_line).map_err(|e| AppError::Ipc(e.to_string()))?;
        let resp: serde_json::Value = serde_json::from_str(&resp_line).map_err(|e| AppError::Ipc(e.to_string()))?;
        Ok(resp)
    }

    pub fn new_run(&mut self, seed: u64, ticks: u64) -> Result<(), AppError> {
        let req = serde_json::json!({ "type": "NewRun", "seed": seed, "ticks": ticks });
        self.send(req)?;
        Ok(())
    }

    pub fn step(&mut self, n: u64) -> Result<PetStateDto, AppError> {
        let req = serde_json::json!({ "type": "Step", "n": n });
        let resp = self.send(req)?;
        if resp["type"] == "PetState" {
            let state: PetStateDto = serde_json::from_value(resp["state"].clone())
                .map_err(|e| AppError::Ipc(e.to_string()))?;
            Ok(state)
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

    pub fn shutdown(self) {
        drop(self);
    }
}
