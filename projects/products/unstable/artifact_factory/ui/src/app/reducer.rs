use crate::app::action::Action;
use crate::app::app_state::{AppState, Screen};

pub fn reduce(state: &mut AppState, action: Action) {
    match action {
        Action::LoadInputs(paths) => {
            state.input_paths = paths;
            state.current_screen = Screen::Graph;
        }
        Action::Analyze => {
            state.current_screen = Screen::Graph;
        }
        Action::RenderDocs => {
            state.current_screen = Screen::Render;
        }
        Action::BuildBundle | Action::GetBundle => {
            state.current_screen = Screen::Bundle;
        }
        Action::Quit => {
            state.running = false;
        }
    }
}
