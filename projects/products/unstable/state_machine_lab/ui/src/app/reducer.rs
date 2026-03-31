use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::screen::Screen;

pub fn reduce(state: &mut AppState, action: Action) {
    match action {
        Action::LoadMachine(_) => {
            state.machine_loaded = true;
            state.current_screen = Screen::Editor;
        }
        Action::Validate => {
            state.validated = true;
            state.current_screen = Screen::Editor;
        }
        Action::Run(_) | Action::Step(_) => {
            state.current_screen = Screen::Run;
        }
        Action::TestExhaustive | Action::TestFuzz { .. } => {
            state.current_screen = Screen::Test;
        }
        Action::GetTranscript => {
            state.current_screen = Screen::Transcript;
        }
        Action::Quit => state.running = false,
    }
}
