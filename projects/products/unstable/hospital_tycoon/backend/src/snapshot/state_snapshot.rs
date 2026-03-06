// projects/products/unstable/hospital_tycoon/backend/src/snapshot/state_snapshot.rs
use crate::model::hospital_state::HospitalState;
use crate::report::tick_report::TickReport;
use crate::sim::event_log::EventLog;
use crate::snapshot::snapshot_hash::SnapshotHash;
use crate::staff::staff_engine::StaffEngine;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    pub tick: u64,
    pub patient_count: usize,
    pub staff_count: usize,
    pub budget_balance: i64,
    pub reputation_score: u32,
    pub patients_treated: u32,
    pub event_count: usize,
    pub hash: String,
    pub data_summary: String,
}

impl StateSnapshot {
    pub fn capture(state: &HospitalState, event_log: &EventLog) -> Self {
        let hash = SnapshotHash::compute(state);
        let tick_report = TickReport {
            tick: state.tick.value(),
            patients_in_queue: state.waiting_patients.len(),
            patients_treated: state.patients_treated(),
            budget_balance: state.budget.balance,
            reputation_score: state.reputation.score,
        };
        let staff_count = StaffEngine::available_count(&state.staff);
        let data_summary = format!(
            "tick={} patients={} queue={} treated={} budget={} rep={}",
            tick_report.tick,
            state.patients.len(),
            tick_report.patients_in_queue,
            tick_report.patients_treated,
            tick_report.budget_balance,
            tick_report.reputation_score
        );
        Self {
            tick: tick_report.tick,
            patient_count: state.patients.len(),
            staff_count,
            budget_balance: tick_report.budget_balance,
            reputation_score: tick_report.reputation_score,
            patients_treated: tick_report.patients_treated,
            event_count: event_log.len(),
            hash,
            data_summary,
        }
    }
}
