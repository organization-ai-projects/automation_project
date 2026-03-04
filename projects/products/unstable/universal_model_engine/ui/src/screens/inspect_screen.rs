use crate::app::app_state::AppState;
use crate::widgets::log_widget::LogWidget;
use crate::widgets::table_widget::TableWidget;

pub struct InspectScreen;

impl InspectScreen {
    pub fn render(state: &AppState) {
        println!("== INSPECT ==");
        let rows = vec![vec![
            "snapshot_hash".to_string(),
            state
                .snapshot_hash
                .clone()
                .unwrap_or_else(|| "<none>".to_string()),
        ]];
        TableWidget::render(&["field", "value"], &rows);
        if let Some(snapshot_json) = &state.snapshot_json {
            LogWidget::render(&[format!("snapshot_json={snapshot_json}")]);
        }
        if let Some(error) = &state.last_error {
            LogWidget::render(&[format!("error: {error}")]);
        }
    }
}
