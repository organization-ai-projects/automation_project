use crate::app::app_state::AppState;
use crate::widgets::table_widget::TableWidget;

pub struct PartyScreen;

impl PartyScreen {
    pub fn render(state: &AppState) {
        println!("== PARTY ==");
        if let Some(ref json) = state.snapshot_json {
            TableWidget::render(
                &["field", "value"],
                &[vec![
                    "snapshot".to_string(),
                    json.chars().take(80).collect(),
                ]],
            );
        }
        if let Some(error) = &state.last_error {
            println!("error: {error}");
        }
    }
}
