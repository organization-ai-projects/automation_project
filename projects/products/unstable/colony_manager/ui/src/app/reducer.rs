// projects/products/unstable/colony_manager/ui/src/app/reducer.rs
use crate::app::action::Action;
use crate::app::app_state::AppState;

pub struct Reducer;

impl Reducer {
    pub fn apply(state: &mut AppState, action: &Action) {
        match action {
            Action::RunRequested => {
                state.running = true;
                state.replaying = false;
            }
            Action::RunCompleted => {
                state.running = false;
            }
            Action::ReplayRequested => {
                state.replaying = true;
                state.running = false;
            }
            Action::ReplayCompleted => {
                state.replaying = false;
            }
        }
    }
}
