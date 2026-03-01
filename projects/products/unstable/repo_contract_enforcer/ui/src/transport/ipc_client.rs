use crate::cli::command::Mode;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use super::backend_process::BackendProcess;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub id: Option<String>,
    #[serde(flatten)]
    pub payload: RequestPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum RequestPayload {
    CheckRepo { root_path: String, mode: Mode },
    CheckProduct { product_path: String, mode: Mode },
    Shutdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub id: Option<String>,
    #[serde(flatten)]
    pub payload: ResponsePayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ResponsePayload {
    Ok,
    Error {
        code: String,
        message: String,
        details: Option<String>,
    },
    Report {
        report_json: serde_json::Value,
        report_hash: String,
        summary: serde_json::Value,
    },
}

pub struct IpcClient {
    backend: BackendProcess,
}

impl IpcClient {
    pub fn connect() -> Result<Self> {
        Ok(Self {
            backend: BackendProcess::spawn()?,
        })
    }

    pub fn request_report(&mut self, payload: RequestPayload) -> Result<Response> {
        let request = Request {
            id: Some("ui-1".to_string()),
            payload,
        };
        let line = serde_json::to_string(&request)?;
        self.backend.send_line(&line)?;

        let response_line = self.backend.read_line()?;
        let response: Response =
            serde_json::from_str(response_line.trim()).context("invalid response from backend")?;
        Ok(response)
    }

    pub fn close(mut self) {
        let request = Request {
            id: Some("ui-shutdown".to_string()),
            payload: RequestPayload::Shutdown,
        };
        if let Ok(line) = serde_json::to_string(&request) {
            let _ = self.backend.send_line(&line);
        }
        self.backend.shutdown();
    }
}
