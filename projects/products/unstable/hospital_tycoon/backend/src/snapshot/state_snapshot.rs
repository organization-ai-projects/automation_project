// projects/products/unstable/hospital_tycoon/backend/src/snapshot/state_snapshot.rs
use crate::model::hospital_state::HospitalState;
use crate::sim::event_log::EventLog;
use crate::snapshot::snapshot_hash::SnapshotHash;
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
        let data_summary = format!(
            "tick={} patients={} treated={} budget={} rep={}",
            state.tick.value(),
            state.patients.len(),
            state.treated_patients.len(),
            state.budget.balance,
            state.reputation.score
        );
        Self {
            tick: state.tick.value(),
            patient_count: state.patients.len(),
            staff_count: state.staff.len(),
            budget_balance: state.budget.balance,
            reputation_score: state.reputation.score,
            patients_treated: state.patients_treated(),
            event_count: event_log.len(),
            hash,
            data_summary,
        }
    }
}
