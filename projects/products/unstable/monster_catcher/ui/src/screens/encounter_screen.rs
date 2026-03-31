use crate::app::app_state::AppState;
use crate::widgets::log_widget::LogWidget;

pub struct EncounterScreen;

impl EncounterScreen {
    pub fn render(state: &AppState) {
        println!("== ENCOUNTER ==");
        if let Some(ref json) = state.encounter_json {
            LogWidget::render(&[format!("encounter={json}")]);
        }
        if let Some(error) = &state.last_error {
            LogWidget::render(&[format!("error: {error}")]);
        }
    }
}
