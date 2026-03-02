use crate::cli::command::Mode;
use anyhow::Result;
use serde::de::{self, Deserializer, Visitor};
use serde::{Deserialize, Serialize};
use std::fmt;

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
    #[serde(deserialize_with = "deserialize_optional_u32")]
    pub line: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReportSummary {
    #[serde(deserialize_with = "deserialize_u64")]
    pub stable_error_count: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    pub stable_warning_count: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    pub unstable_error_count: u64,
    #[serde(deserialize_with = "deserialize_u64")]
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

fn deserialize_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    struct U64Visitor;

    impl<'de> Visitor<'de> for U64Visitor {
        type Value = u64;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a non-negative integer number")
        }

        fn visit_u64<E>(self, value: u64) -> Result<u64, E>
        where
            E: de::Error,
        {
            Ok(value)
        }

        fn visit_i64<E>(self, value: i64) -> Result<u64, E>
        where
            E: de::Error,
        {
            if value < 0 {
                return Err(E::custom("negative value is not allowed"));
            }
            Ok(value as u64)
        }

        fn visit_f64<E>(self, value: f64) -> Result<u64, E>
        where
            E: de::Error,
        {
            if value < 0.0 || value.fract() != 0.0 {
                return Err(E::custom("non-integer or negative float is not allowed"));
            }
            Ok(value as u64)
        }
    }

    deserializer.deserialize_any(U64Visitor)
}

fn deserialize_optional_u32<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum OptionalNumber {
        U64(u64),
        I64(i64),
        F64(f64),
        Null,
    }

    match OptionalNumber::deserialize(deserializer)? {
        OptionalNumber::Null => Ok(None),
        OptionalNumber::U64(v) => u32::try_from(v)
            .map(Some)
            .map_err(|_| de::Error::custom("value out of range for u32")),
        OptionalNumber::I64(v) => {
            if v < 0 {
                return Err(de::Error::custom("negative value is not allowed"));
            }
            u32::try_from(v as u64)
                .map(Some)
                .map_err(|_| de::Error::custom("value out of range for u32"))
        }
        OptionalNumber::F64(v) => {
            if v < 0.0 || v.fract() != 0.0 {
                return Err(de::Error::custom(
                    "non-integer or negative float is not allowed",
                ));
            }
            u32::try_from(v as u64)
                .map(Some)
                .map_err(|_| de::Error::custom("value out of range for u32"))
        }
    }
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
        let trimmed = response_line.trim();
        let response: Response = common_json::from_json_str(trimmed).map_err(|err| {
            anyhow::anyhow!("invalid response from backend: {trimmed}; parse error: {err}")
        })?;
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
