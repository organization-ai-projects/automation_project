use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::screen::Screen;

pub fn reduce(state: &mut AppState, action: Action) {
    match action {
        Action::LoadModel(_) => {
            state.model_loaded = true;
            state.current_screen = Screen::Dsl;
        }
        Action::ValidateModel => {
            state.current_screen = Screen::Dsl;
        }
        Action::NewRun { .. } | Action::Step | Action::RunToEnd => {
            state.current_screen = Screen::Run;
        }
        Action::GetSnapshot => {
            state.current_screen = Screen::Inspect;
        }
        Action::SaveReplay | Action::LoadReplay(_) | Action::ReplayToEnd => {
            state.current_screen = Screen::Replay;
        }
        Action::GetReport => {
            state.current_screen = Screen::Report;
        }
        Action::Quit => state.running = false,
    }
}
