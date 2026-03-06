use crate::app::app_state::AppState;
use crate::widgets::log_widget::LogWidget;
use crate::widgets::table_widget::TableWidget;

pub struct ReportScreen;

impl ReportScreen {
    pub fn render(state: &AppState) {
        println!("== REPORT ==");
        let rows = vec![vec![
            "run_hash".to_string(),
            state
                .run_hash
                .clone()
                .unwrap_or_else(|| "<none>".to_string()),
        ]];
        TableWidget::render(&["field", "value"], &rows);
        if let Some(report) = &state.last_report {
            LogWidget::render(&[format!("report_json={report}")]);
        }
        if let Some(error) = &state.last_error {
            LogWidget::render(&[format!("error: {error}")]);
        }
    }
}
