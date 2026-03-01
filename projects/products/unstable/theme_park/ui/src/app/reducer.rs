#![allow(dead_code)]
use crate::app::action::Action;
use crate::app::app_state::AppState;

/// Pure state reducer (no side effects).
pub struct Reducer;

impl Reducer {
    pub fn reduce(state: &AppState, action: &Action) -> AppState {
        let mut next = state.clone();
        match action {
            Action::SetScenario(s) => next.scenario_path = Some(s.clone()),
            Action::SetSeed(s) => next.seed = *s,
            Action::SetTicks(t) => next.ticks = *t,
            Action::RunComplete { report_json } => {
                next.last_report = Some(report_json.clone());
                next.run_complete = true;
                next.last_error = None;
            }
            Action::Error(e) => next.last_error = Some(e.clone()),
            Action::Reset => {
                next.last_report = None;
                next.last_error = None;
                next.run_complete = false;
            }
        }
        next
    }
}
