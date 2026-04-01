// projects/products/unstable/spec_to_runtime_compiler/ui/src/screens/report_screen.rs
use crate::app::app_state::AppState;

pub struct ReportScreen;

impl ReportScreen {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, state: &AppState) {
        tracing::info!(report = ?state.last_report, "ReportScreen rendered");
    }
}
