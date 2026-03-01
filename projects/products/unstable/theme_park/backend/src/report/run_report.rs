#![allow(dead_code)]
use crate::events::event_log::EventLog;
use crate::report::run_hash::RunHash;
use crate::report::tick_report::TickReport;
use crate::sim::sim_state::SimState;
use crate::snapshot::state_snapshot::StateSnapshot;
use serde::{Deserialize, Serialize};

/// The canonical, fully-deterministic report for a completed run.
/// Canonical JSON field order must be maintained (serde serialize order = declaration order).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunReport {
    pub scenario_id: String,
    pub scenario_hash: String,
    pub seed: u64,
    pub ticks: u64,
    pub run_hash: String,
    pub visitors_served: u32,
    pub total_visitors: usize,
    pub average_wait: f64,
    pub total_revenue: u64,
    pub final_reputation: i32,
    pub event_count: usize,
    pub snapshot_checkpoints: Vec<TickReport>,
}

impl RunReport {
    /// Build a canonical report from the final state and event log.
    pub fn build(state: &SimState, event_log: &EventLog, ticks: u64) -> Self {
        let scenario_id = String::from("theme_park");
        let scenario_hash = String::from("unknown");
        let seed = 0u64;
        Self::build_with_scenario(state, event_log, ticks, &scenario_id, &scenario_hash, seed)
    }

    pub fn build_with_scenario(
        state: &SimState,
        event_log: &EventLog,
        ticks: u64,
        scenario_id: &str,
        scenario_hash: &str,
        seed: u64,
    ) -> Self {
        let checkpoints: Vec<TickReport> = if state.config.snapshot_checkpoints {
            // Single checkpoint at the final state.
            let snap = StateSnapshot::from_state(state);
            vec![TickReport::new(
                state.clock.tick,
                snap.active_visitor_count,
                snap.total_revenue,
                snap.reputation_score,
                snap.hash,
            )]
        } else {
            vec![]
        };

        let run_hash = RunHash::from_event_log(event_log, scenario_hash, ticks);

        Self {
            scenario_id: scenario_id.to_string(),
            scenario_hash: scenario_hash.to_string(),
            seed,
            ticks,
            run_hash,
            visitors_served: state.total_visitors_served(),
            total_visitors: state.visitors.len(),
            average_wait: state.average_wait_ticks(),
            total_revenue: state.total_revenue(),
            final_reputation: state.reputation.score,
            event_count: event_log.len(),
            snapshot_checkpoints: checkpoints,
        }
    }
}
