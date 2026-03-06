use crate::app::app_state::AppState;
use crate::widgets::log_widget::LogWidget;
use crate::widgets::table_widget::TableWidget;

pub struct ReplayScreen;

impl ReplayScreen {
    pub fn render(state: &AppState) {
        println!("== REPLAY ==");
        let rows = vec![
            vec!["replay_saved".to_string(), state.replay_saved.to_string()],
            vec![
                "replay_available".to_string(),
                state.replay_data.is_some().to_string(),
            ],
        ];
        TableWidget::render(&["field", "value"], &rows);
        if let Some(replay_data) = &state.replay_data {
            LogWidget::render(&[format!("replay_bytes={}", replay_data.len())]);
        }
        if let Some(error) = &state.last_error {
            LogWidget::render(&[format!("error: {error}")]);
        }
    }
}
