// projects/products/unstable/protocol_builder/ui/src/transport/ipc_client.rs
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Write};

use super::backend_process::BackendProcess;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum IpcRequest {
    LoadSchema { path: String },
    ValidateSchema,
    GenerateDryRun,
    GenerateWrite { out_dir: String },
    GetReport,
    Shutdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum IpcResponse {
    Ok,
    Error {
        message: String,
    },
    GenerateReport {
        manifest_hash: String,
        report_json: String,
    },
}

pub struct IpcClient {
    process: BackendProcess,
}

impl IpcClient {
    pub fn new(process: BackendProcess) -> Self {
        Self { process }
    }

    fn send(&mut self, req: &IpcRequest) -> Result<IpcResponse> {
        let line = common_json::to_string(req)?;
        writeln!(self.process.stdin, "{}", line)?;
        self.process.stdin.flush()?;

        let mut buf = String::new();
        self.process.reader.read_line(&mut buf)?;
        let response: IpcResponse = common_json::from_json_str(buf.trim())?;
        Ok(response)
    }

    pub fn send_load_schema(&mut self, path: &str) -> Result<()> {
        match self.send(&IpcRequest::LoadSchema {
            path: path.to_string(),
        })? {
            IpcResponse::Ok => Ok(()),
            IpcResponse::Error { message } => Err(anyhow!("LoadSchema error: {}", message)),
            _ => Err(anyhow!("unexpected response")),
        }
    }

    pub fn send_validate(&mut self) -> Result<()> {
        match self.send(&IpcRequest::ValidateSchema)? {
            IpcResponse::Ok => Ok(()),
            IpcResponse::Error { message } => Err(anyhow!("ValidateSchema error: {}", message)),
            _ => Err(anyhow!("unexpected response")),
        }
    }

    pub fn send_generate_write(&mut self, out_dir: &str) -> Result<(String, String)> {
        match self.send(&IpcRequest::GenerateWrite {
            out_dir: out_dir.to_string(),
        })? {
            IpcResponse::GenerateReport {
                manifest_hash,
                report_json,
            } => Ok((manifest_hash, report_json)),
            IpcResponse::Error { message } => Err(anyhow!("GenerateWrite error: {}", message)),
            _ => Err(anyhow!("unexpected response")),
        }
    }

    pub fn send_shutdown(&mut self) -> Result<()> {
        let line = common_json::to_string(&IpcRequest::Shutdown)?;
        let _ = writeln!(self.process.stdin, "{}", line);
        let _ = self.process.stdin.flush();
        Ok(())
    }
}
