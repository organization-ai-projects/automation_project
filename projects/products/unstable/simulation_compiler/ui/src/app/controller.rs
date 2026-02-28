// projects/products/unstable/simulation_compiler/ui/src/app/controller.rs
use super::action::Action;
use super::app_state::AppState;
use super::reducer::apply;

pub struct Controller {
    pub state: AppState,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            state: AppState::default(),
        }
    }

    pub fn dispatch(&mut self, action: Action) {
        apply(&mut self.state, action);
    }
}
