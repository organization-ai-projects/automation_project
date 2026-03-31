use crate::state::app_state::AppState;

pub struct App {
    state: AppState,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::new(),
        }
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }

    pub fn update_status(&mut self, status: String) {
        self.state.set_status(status);
    }

    pub fn set_report(&mut self, report_json: String) {
        self.state.set_report(report_json);
    }
}
