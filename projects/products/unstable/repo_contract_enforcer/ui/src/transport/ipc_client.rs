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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReportMode {
    Auto,
    Strict,
    Relaxed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Scope {
    Stable,
    Unstable,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Violation {
    pub rule_id: String,
    pub violation_code: String,
    pub severity: Severity,
    pub scope: Scope,
    pub path: String,
    pub message: String,
    pub line: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReportSummary {
    pub stable_error_count: u64,
    pub stable_warning_count: u64,
    pub unstable_error_count: u64,
    pub unstable_warning_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Report {
    pub repository_root: String,
    pub mode: ReportMode,
    pub violations: Vec<Violation>,
    pub summary: ReportSummary,
    pub report_hash: String,
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
        report_json: Report,
        report_hash: String,
        summary: ReportSummary,
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
        let line = common_json::to_string(&request)?;
        self.backend.send_line(&line)?;

        let response_line = self.backend.read_line()?;
        let response: Response = common_json::from_json_str(response_line.trim())
            .context("invalid response from backend")?;
        Ok(response)
    }

    pub fn close(mut self) {
        let request = Request {
            id: Some("ui-shutdown".to_string()),
            payload: RequestPayload::Shutdown,
        };
        if let Ok(line) = common_json::to_string(&request) {
            let _ = self.backend.send_line(&line);
        }
        self.backend.shutdown();
    }
}
