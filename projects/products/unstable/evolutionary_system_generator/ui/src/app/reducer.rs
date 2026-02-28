use crate::app::app_state::{AppState, Screen};
use serde_json::Value;

pub fn apply_report(state: &mut AppState, generation: u32, best_fitness: f64, done: bool) {
    state.generation = generation;
    state.best_fitness = best_fitness;
    state.done = done;
    state.current_screen = Screen::Running;
}

pub fn apply_candidates(state: &mut AppState, manifest: Value) {
    state.last_manifest = Some(manifest);
    state.current_screen = Screen::Candidates;
}

pub fn apply_error(state: &mut AppState, message: String) {
    state.error = Some(message);
}
