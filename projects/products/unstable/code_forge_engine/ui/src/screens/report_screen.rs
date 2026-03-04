use crate::app::app_state::AppState;
use crate::widgets::diff_widget::DiffWidget;
use crate::widgets::table_widget::TableWidget;

pub struct ReportScreen;

impl ReportScreen {
    pub fn render(state: &AppState) {
        let rows = vec![vec![
            "manifest_hash".to_string(),
            state
                .manifest_hash
                .clone()
                .unwrap_or_else(|| "<none>".to_string()),
        ]];
        TableWidget::render(&["field", "value"], &rows);

        let manifest = state
            .manifest_json
            .clone()
            .unwrap_or_else(|| "{}".to_string());
        DiffWidget::render(&manifest, &manifest);
    }
}
