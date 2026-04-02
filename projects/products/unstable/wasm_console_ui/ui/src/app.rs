use crate::state::app_state::AppState as UiAppState;
use crate::transport::ipc_client::IpcClient;

/// Main application entry point for the UI.
pub struct App {
    state: UiAppState,
    client: IpcClient,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: UiAppState::new(),
            client: IpcClient::new(),
        }
    }

    pub fn state(&self) -> &UiAppState {
        &self.state
    }

    pub fn client(&self) -> &IpcClient {
        &self.client
    }

    pub fn update_state(&mut self, new_state: UiAppState) {
        self.state = new_state;
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

/// Dioxus app function (only meaningful under wasm32).
#[cfg(target_arch = "wasm32")]
pub fn app() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        div {
            class: "wasm-console-ui",
            h1 { "Patchsmith Console" }
            p { "WASM Console UI Shell" }
        }
    }
}
