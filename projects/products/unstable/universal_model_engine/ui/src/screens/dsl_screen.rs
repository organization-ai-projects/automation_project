use crate::app::app_state::AppState;
use crate::widgets::log_widget::LogWidget;
use crate::widgets::table_widget::TableWidget;

pub struct DslScreen;

impl DslScreen {
    pub fn render(state: &AppState) {
        println!("== DSL ==");
        let rows = vec![vec![
            "model_loaded".to_string(),
            state.model_loaded.to_string(),
        ]];
        TableWidget::render(&["field", "value"], &rows);
        if let Some(error) = &state.last_error {
            LogWidget::render(&[format!("error: {error}")]);
        }
    }
}
