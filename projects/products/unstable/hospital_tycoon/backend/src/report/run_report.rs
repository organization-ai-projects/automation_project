// projects/products/unstable/hospital_tycoon/backend/src/report/run_report.rs
use crate::model::hospital_state::HospitalState;
use crate::report::run_hash::RunHash;
use crate::sim::event_log::EventLog;
use crate::time::tick_clock::TickClock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunReport {
    pub seed: u64,
    pub scenario_name: String,
    pub total_ticks: u64,
    pub patients_treated: u32,
    pub patients_died: u32,
    pub final_budget: i64,
    pub final_reputation: u32,
    pub event_count: usize,
    pub run_hash: String,
}

impl RunReport {
    pub fn generate(
        state: &HospitalState,
        clock: &TickClock,
        event_log: &EventLog,
        scenario_name: &str,
    ) -> Self {
        let patients_treated = state.patients_treated();
        let patients_died = 0u32;
        let final_budget = state.budget.balance;
        let final_reputation = state.reputation.score;
        let event_count = event_log.len();

        let run_hash = RunHash::compute(
            clock.seed,
            scenario_name,
            clock.current_tick().value(),
            patients_treated,
            patients_died,
            final_budget,
            final_reputation,
            event_count,
        );

        Self {
            seed: clock.seed,
            scenario_name: scenario_name.to_string(),
            total_ticks: clock.current_tick().value(),
            patients_treated,
            patients_died,
            final_budget,
            final_reputation,
            event_count,
            run_hash,
        }
    }

    /// Canonical JSON encoding for deterministic hashing (sorted keys via BTreeMap).
    pub fn canonical_json(&self) -> String {
        use std::collections::BTreeMap;
        let mut map = BTreeMap::new();
        map.insert("seed", serde_json::json!(self.seed));
        map.insert("scenario_name", serde_json::json!(self.scenario_name));
        map.insert("total_ticks", serde_json::json!(self.total_ticks));
        map.insert("patients_treated", serde_json::json!(self.patients_treated));
        map.insert("patients_died", serde_json::json!(self.patients_died));
        map.insert("final_budget", serde_json::json!(self.final_budget));
        map.insert("final_reputation", serde_json::json!(self.final_reputation));
        map.insert("event_count", serde_json::json!(self.event_count));
        map.insert("run_hash", serde_json::json!(self.run_hash));
        serde_json::to_string(&map).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_report() -> RunReport {
        RunReport {
            seed: 42,
            scenario_name: "test".to_string(),
            total_ticks: 50,
            patients_treated: 5,
            patients_died: 0,
            final_budget: 11500,
            final_reputation: 55,
            event_count: 20,
            run_hash: "abc123".to_string(),
        }
    }

    #[test]
    fn canonical_json_is_deterministic() {
        let r = make_report();
        let j1 = r.canonical_json();
        let j2 = r.canonical_json();
        assert_eq!(j1, j2);
    }
}
