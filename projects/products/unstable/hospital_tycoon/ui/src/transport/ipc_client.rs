// projects/products/unstable/hospital_tycoon/ui/src/transport/ipc_client.rs
use crate::diagnostics::app_error::AppError;
use crate::transport::backend_process::BackendProcess;
use crate::transport::outbound_message::OutboundMessage;
use crate::transport::outbound_request::OutboundRequest;
use crate::transport::run_report_dto::RunReportDto;
use crate::transport::snapshot_dto::SnapshotDto;
use common_json::JsonAccess;
use std::io::{BufRead, BufReader, Write};

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
