// projects/products/unstable/simulation_compiler/ui/src/screens/compile_screen.rs
use crate::app::app_state::AppState;

pub struct CompileScreen;

impl CompileScreen {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, state: &AppState) {
        tracing::info!(report = ?state.last_report, "CompileScreen rendered");
    }
}
