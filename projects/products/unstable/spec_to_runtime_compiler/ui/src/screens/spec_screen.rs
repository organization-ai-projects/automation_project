// projects/products/unstable/spec_to_runtime_compiler/ui/src/screens/spec_screen.rs
use crate::app::app_state::AppState;

pub struct SpecScreen {
    pub path: String,
}

impl SpecScreen {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }

    pub fn render(&self, state: &AppState) {
        tracing::info!(path = %self.path, error = ?state.error, "SpecScreen rendered");
    }
}
