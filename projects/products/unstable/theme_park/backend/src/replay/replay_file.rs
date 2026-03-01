#![allow(dead_code)]
use crate::events::event_log::EventLog;
use crate::scenario::scenario::Scenario;
use crate::sim::sim_state::SimState;
use serde::{Deserialize, Serialize};

/// Persisted replay file containing everything needed to replay a run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayFile {
    pub scenario_id: String,
    pub scenario_hash: String,
    pub seed: u64,
    pub ticks: u64,
    pub event_log_checksum: u64,
    pub event_log_len: usize,
    pub scenario_json: String,
}

impl ReplayFile {
    pub fn from_run(scenario: &Scenario, state: &SimState, event_log: &EventLog) -> Self {
        let scenario_json =
            serde_json::to_string(scenario).unwrap_or_default();
        let scenario_hash = scenario.hash();
        Self {
            scenario_id: scenario.id.clone(),
            scenario_hash,
            seed: scenario.seed,
            ticks: state.clock.tick.value(),
            event_log_checksum: event_log.checksum(),
            event_log_len: event_log.len(),
            scenario_json,
        }
    }
}
