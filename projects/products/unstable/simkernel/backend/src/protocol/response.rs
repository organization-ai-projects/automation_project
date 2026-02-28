#![allow(dead_code)]
use crate::report::run_report::RunReport;
use crate::snapshot::state_snapshot::StateSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Response {
    Ok { id: u64 },
    Error { id: Option<u64>, code: String, message: String, details: String },
    Packs { id: u64, packs: Vec<String> },
    ScenarioValidated { id: u64, scenario_hash: String },
    Snapshot { id: u64, snapshot_hash: String, tick: u64, turn: u64, data_summary: String },
    QueryReport { id: u64, report_json: String },
    Report { id: u64, run_hash: String, report_json: String },
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

    pub fn packs(id: u64, packs: Vec<String>) -> Self {
        Self::Packs { id, packs }
    }

    pub fn report(id: u64, report: &RunReport) -> Self {
        let run_hash = report.run_hash.clone();
        let report_json = serde_json::to_string(report).unwrap_or_default();
        Self::Report { id, run_hash, report_json }
    }

    pub fn snapshot(id: u64, snapshot: &StateSnapshot, at_tick: u64, at_turn: u64) -> Self {
        Self::Snapshot {
            id,
            snapshot_hash: snapshot.hash.clone(),
            tick: at_tick,
            turn: at_turn,
            data_summary: format!("{} entities", snapshot.entity_count),
        }
    }
}
