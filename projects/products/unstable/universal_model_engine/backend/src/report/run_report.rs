use crate::events::event_log::EventLog;
use crate::model::state::State;
use crate::solve::solve_report::SolveReport;
use crate::solve::solve_step::SolveStep;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunReport {
    pub seed: u64,
    pub step_count: u64,
    pub state: State,
    pub event_count: usize,
    pub solve_report: SolveReport,
}

impl RunReport {
    pub fn from_state(seed: u64, step_count: u64, state: &State, event_log: &EventLog) -> Self {
        let steps = event_log
            .events
            .iter()
            .map(|event| SolveStep {
                index: event.step,
                transition_id: event.transition_id.clone(),
            })
            .collect();

        Self {
            seed,
            step_count,
            state: state.clone(),
            event_count: event_log.events.len(),
            solve_report: SolveReport { steps },
        }
    }
}
