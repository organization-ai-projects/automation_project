// projects/products/unstable/hospital_tycoon/backend/src/protocol/response.rs
use crate::report::run_report::RunReport;
use crate::snapshot::state_snapshot::StateSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Response {
    Ok {
        id: Option<u64>,
    },
    Error {
        id: Option<u64>,
        code: String,
        message: String,
        details: String,
    },
    Snapshot {
        id: Option<u64>,
        hash: String,
        tick: u64,
        data_summary: String,
        snapshot: StateSnapshot,
    },
    Report {
        id: Option<u64>,
        run_hash: String,
        report_json: String,
        report: RunReport,
    },
}

impl Response {
    pub fn ok(id: Option<u64>) -> Self {
        Self::Ok { id }
    }

    pub fn error(id: Option<u64>, code: &str, message: &str) -> Self {
        Self::Error {
            id,
            code: code.to_string(),
            message: message.to_string(),
            details: String::new(),
        }
    }

    pub fn snapshot(id: Option<u64>, snap: StateSnapshot) -> Self {
        let hash = snap.hash.clone();
        let tick = snap.tick;
        let data_summary = snap.data_summary.clone();
        Self::Snapshot { id, hash, tick, data_summary, snapshot: snap }
    }

    pub fn report(id: Option<u64>, report: RunReport) -> Self {
        let run_hash = report.run_hash.clone();
        let report_json = serde_json::to_string(&report).unwrap_or_default();
        Self::Report { id, run_hash, report_json, report }
    }
}
