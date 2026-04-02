//! projects/products/unstable/market_tycoon/ui/src/app.rs
use crate::state::AppState;
use dioxus::prelude::*;

pub(crate) struct App {
    state: AppState,
}

impl App {
    pub(crate) fn new() -> Self {
        Self {
            state: AppState::new(),
        }
    }

    pub(crate) fn state(&self) -> &AppState {
        &self.state
    }

    pub(crate) fn update_status(&mut self, status: String) {
        self.state.set_status(status);
    }

    pub(crate) fn set_report(&mut self, report_json: String) {
        self.state.set_report(report_json);
    }
}

pub(crate) fn app() -> Element {
    rsx!(
        div {
            h1 { "Market Tycoon UI" }
            p { "Welcome to the application!" }
        }
    )
}
