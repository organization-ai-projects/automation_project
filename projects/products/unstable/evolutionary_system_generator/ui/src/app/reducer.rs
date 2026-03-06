// projects/products/unstable/evolutionary_system_generator/ui/src/app/reducer.rs
use crate::app::app_state::AppState;
use crate::app::screen::Screen;
use common_json::Json;

pub fn apply_report(state: &mut AppState, generation: u32, best_fitness: f64, done: bool) {
    state.generation = generation;
    state.best_fitness = best_fitness;
    state.done = done;
    state.current_screen = if done {
        Screen::Report
    } else {
        Screen::Running
    };
}

pub fn apply_candidates(state: &mut AppState, manifest: Json) {
    state.last_manifest = Some(manifest);
    state.current_screen = Screen::Candidates;
}

pub fn apply_error(state: &mut AppState, message: String) {
    state.error = Some(message);
}
