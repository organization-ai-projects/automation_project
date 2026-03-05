// projects/products/unstable/hospital_tycoon/backend/src/report/run_report.rs
use crate::model::hospital_state::HospitalState;
use crate::report::run_hash::RunHash;
use crate::report::run_hash_input::RunHashInput;
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

        let run_hash_input = RunHashInput {
            seed: clock.seed,
            scenario_name,
            total_ticks: clock.current_tick().value(),
            patients_treated,
            patients_died,
            final_budget,
            final_reputation,
            event_count,
        };
        let run_hash = RunHash::compute(&run_hash_input);

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
        let seed = common_json::to_json_string(&self.seed).unwrap_or_else(|_| "0".to_string());
        let scenario_name =
            common_json::to_json_string(&self.scenario_name).unwrap_or_else(|_| "\"\"".to_string());
        let total_ticks =
            common_json::to_json_string(&self.total_ticks).unwrap_or_else(|_| "0".to_string());
        let patients_treated = common_json::to_json_string(&(self.patients_treated as u64))
            .unwrap_or_else(|_| "0".to_string());
        let patients_died = common_json::to_json_string(&(self.patients_died as u64))
            .unwrap_or_else(|_| "0".to_string());
        let final_budget =
            common_json::to_json_string(&self.final_budget).unwrap_or_else(|_| "0".to_string());
        let final_reputation = common_json::to_json_string(&(self.final_reputation as u64))
            .unwrap_or_else(|_| "0".to_string());
        let event_count = common_json::to_json_string(&(self.event_count as u64))
            .unwrap_or_else(|_| "0".to_string());
        let run_hash =
            common_json::to_json_string(&self.run_hash).unwrap_or_else(|_| "\"\"".to_string());

        format!(
            "{{\"event_count\":{event_count},\"final_budget\":{final_budget},\"final_reputation\":{final_reputation},\"patients_died\":{patients_died},\"patients_treated\":{patients_treated},\"run_hash\":{run_hash},\"scenario_name\":{scenario_name},\"seed\":{seed},\"total_ticks\":{total_ticks}}}"
        )
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
