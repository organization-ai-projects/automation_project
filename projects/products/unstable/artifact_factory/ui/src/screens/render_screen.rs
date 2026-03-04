use crate::app::app_state::AppState;
use crate::widgets::table_widget::TableWidget;

pub struct RenderScreen;

impl RenderScreen {
    pub fn render(state: &AppState) {
        println!("=== Render Screen ===");
        if let Some(ref err) = state.last_error {
            println!("Error: {err}");
        } else {
            let rows = vec![
                vec![
                    "docs.md bytes".to_string(),
                    state.markdown_bytes.to_string(),
                ],
                vec!["graph.svg bytes".to_string(), state.svg_bytes.to_string()],
                vec!["docs.html bytes".to_string(), state.html_bytes.to_string()],
            ];
            TableWidget::render(&["Artifact", "Size"], &rows);
        }
    }
}
