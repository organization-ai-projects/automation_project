use crate::app::app_state::AppState;
use crate::widgets::log_widget::LogWidget;
use crate::widgets::table_widget::TableWidget;

pub struct RunScreen;

impl RunScreen {
    pub fn render(state: &AppState) {
        println!("== RUN ==");
        let rows = vec![
            vec!["replay_saved".to_string(), state.replay_saved.to_string()],
            vec!["running".to_string(), state.running.to_string()],
        ];
        TableWidget::render(&["field", "value"], &rows);
        if let Some(error) = &state.last_error {
            LogWidget::render(&[format!("error: {error}")]);
        }
    }
}
