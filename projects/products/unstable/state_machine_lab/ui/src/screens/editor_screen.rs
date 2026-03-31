use crate::app::app_state::AppState;
use crate::widgets::log_widget::LogWidget;
use crate::widgets::table_widget::TableWidget;

pub struct EditorScreen;

impl EditorScreen {
    pub fn render(state: &AppState) {
        println!("== EDITOR ==");
        let rows = vec![
            vec!["machine_loaded".to_string(), state.machine_loaded.to_string()],
            vec!["validated".to_string(), state.validated.to_string()],
        ];
        TableWidget::render(&["field", "value"], &rows);
        if let Some(error) = &state.last_error {
            LogWidget::render(&[format!("error: {error}")]);
        }
    }
}
