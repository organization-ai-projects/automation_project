#![allow(dead_code)]
use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::reducer::Reducer;

pub struct AppController {
    state: AppState,
}

impl AppController {
    pub fn new() -> Self { Self { state: AppState::new() } }

    pub fn dispatch(&mut self, action: Action) {
        self.state = Reducer::reduce(&self.state, &action);
    }

    pub fn state(&self) -> &AppState { &self.state }
}
