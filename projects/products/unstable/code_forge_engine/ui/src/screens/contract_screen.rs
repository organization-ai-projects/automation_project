use crate::app::app_state::AppState;
use crate::widgets::table_widget::TableWidget;

pub struct ContractScreen;

impl ContractScreen {
    pub fn render(state: &AppState) {
        let rows = vec![vec![
            "contract".to_string(),
            state
                .contract_path
                .clone()
                .unwrap_or_else(|| "<none>".to_string()),
        ]];
        TableWidget::render(&["field", "value"], &rows);
    }
}
