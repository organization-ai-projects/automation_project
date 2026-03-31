use crate::app::app_state::AppState;
use crate::widgets::log_widget::LogWidget;

pub struct TranscriptScreen;

impl TranscriptScreen {
    pub fn render(state: &AppState) {
        println!("== TRANSCRIPT ==");
        if let Some(transcript) = &state.transcript {
            LogWidget::render(&[format!("transcript: {transcript}")]);
        }
        if let Some(error) = &state.last_error {
            LogWidget::render(&[format!("error: {error}")]);
        }
    }
}
