use crate::app::action::Action;
use crate::app::app_state::AppState;

pub struct Reducer;

impl Reducer {
    pub fn apply(state: &mut AppState, action: Action) {
        match action {
            Action::Started(command) => {
                state.last_command = Some(command);
                state.last_error = None;
            }
            Action::Finished(code) => {
                state.last_exit_code = Some(code);
            }
            Action::Failed(error) => {
                state.last_error = Some(error);
                state.last_exit_code = Some(5);
            }
        }
    }
}
