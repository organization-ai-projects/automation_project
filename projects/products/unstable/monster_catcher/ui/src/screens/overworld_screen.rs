use crate::app::app_state::AppState;
use crate::widgets::log_widget::LogWidget;

pub struct OverworldScreen;

impl OverworldScreen {
    pub fn render(state: &AppState) {
        println!("== OVERWORLD ==");
        let mut lines = Vec::new();
        lines.push(format!("scenario_loaded={}", state.scenario_loaded));
        lines.push(format!("run_active={}", state.run_active));
        if let Some(error) = &state.last_error {
            lines.push(format!("error: {error}"));
        }
        LogWidget::render(&lines);
    }
}
