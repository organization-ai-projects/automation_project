use crate::app::app_state::AppState;
use crate::widgets::table_widget::TableWidget;

pub struct InputScreen;

impl InputScreen {
    pub fn render(state: &AppState) {
        println!("=== Input Screen ===");
        if state.input_paths.is_empty() {
            println!("No inputs loaded.");
        } else {
            let rows: Vec<Vec<String>> = state
                .input_paths
                .iter()
                .enumerate()
                .map(|(index, path)| vec![(index + 1).to_string(), path.clone()])
                .collect();
            TableWidget::render(&["#", "Path"], &rows);
        }
    }
}
