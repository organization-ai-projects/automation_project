// projects/products/unstable/digital_pet/ui/src/transport/ipc_client.rs
use crate::diagnostics::app_error::AppError;
use crate::transport::backend_process::BackendProcess;
use crate::transport::message::Message;
use crate::transport::pet_state_dto::PetStateDto;
use crate::transport::request::Request;
use crate::transport::response::Response;
use crate::transport::run_report_dto::RunReportDto;
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

    fn send(&mut self, request: Request) -> Result<Response, AppError> {
        let id = self.next_id;
        self.next_id += 1;
        let msg = Message {
            id: Some(id),
            request,
        };
        let line = common_json::to_string(&msg).map_err(|e| AppError::Ipc(e.to_string()))?;
        writeln!(self.stdin, "{}", line).map_err(|e| AppError::Ipc(e.to_string()))?;
        self.stdin
            .flush()
            .map_err(|e| AppError::Ipc(e.to_string()))?;
        let mut resp_line = String::new();
        self.reader
            .read_line(&mut resp_line)
            .map_err(|e| AppError::Ipc(e.to_string()))?;
        let resp: Response =
            common_json::from_str(&resp_line).map_err(|e| AppError::Ipc(e.to_string()))?;
        Ok(resp)
    }

    pub fn new_run(&mut self, seed: u64, ticks: u64) -> Result<(), AppError> {
        self.send(Request::NewRun { seed, ticks })?;
        Ok(())
    }

    pub fn step(&mut self, n: u64) -> Result<PetStateDto, AppError> {
        match self.send(Request::Step { n })? {
            Response::PetState { state, .. } => Ok(state),
            Response::Error { message, .. } => Err(AppError::Ipc(message)),
            _ => Err(AppError::Ipc("unexpected response for Step".to_string())),
        }
    }

    pub fn get_report(&mut self) -> Result<RunReportDto, AppError> {
        match self.send(Request::GetReport)? {
            Response::Report { report, .. } => Ok(report),
            Response::Error { message, .. } => Err(AppError::Ipc(message)),
            _ => Err(AppError::Ipc(
                "unexpected response for GetReport".to_string(),
            )),
        }
    }

    pub fn shutdown(self) {
        drop(self);
    }
}
