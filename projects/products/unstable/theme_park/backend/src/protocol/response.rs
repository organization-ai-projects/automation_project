#![allow(dead_code)]
use crate::report::run_report::RunReport;
use crate::snapshot::state_snapshot::StateSnapshot;
use serde::{Deserialize, Serialize};

/// All response kinds the backend can emit.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Response {
    Ok {
        id: u64,
    },
    Error {
        id: Option<u64>,
        code: String,
        message: String,
        details: String,
    },
    Snapshot {
        id: u64,
        hash: String,
        tick: u64,
        data_summary: String,
    },
    Report {
        id: u64,
        run_hash: String,
        report_json: serde_json::Value,
    },
}

impl Response {
    pub fn ok(id: u64) -> Self {
        Self::Ok { id }
    }

    pub fn error(id: Option<u64>, code: &str, message: &str, details: &str) -> Self {
        Self::Error {
            id,
            code: code.to_string(),
            message: message.to_string(),
            details: details.to_string(),
        }
    }

    pub fn snapshot(id: u64, snap: &StateSnapshot) -> Self {
        Self::Snapshot {
            id,
            hash: snap.hash.clone(),
            tick: snap.tick,
            data_summary: format!(
                "tick={} visitors={} rides={}",
                snap.tick, snap.visitor_count, snap.ride_count
            ),
        }
    }

    pub fn report(id: u64, report: &RunReport) -> Self {
        let run_hash = report.run_hash.clone();
        let report_json =
            serde_json::to_value(report).unwrap_or(serde_json::Value::Null);
        Self::Report {
            id,
            run_hash,
            report_json,
        }
    }
}
