// projects/products/unstable/simulation_compiler/ui/src/screens/dsl_screen.rs
use crate::app::app_state::AppState;

pub struct DslScreen {
    pub path: String,
}

impl DslScreen {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }

    pub fn render(&self, state: &AppState) {
        tracing::info!(path = %self.path, error = ?state.error, "DslScreen rendered");
    }
}
