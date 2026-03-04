use crate::app::app_state::AppState;
use crate::widgets::table_widget::TableWidget;

pub struct GraphScreen;

impl GraphScreen {
    pub fn render(state: &AppState) {
        println!("=== Graph Screen ===");
        let rows = vec![
            vec!["Inputs".to_string(), state.inputs_total.to_string()],
            vec!["Events".to_string(), state.events_count.to_string()],
            vec!["Protocols".to_string(), state.protocols_count.to_string()],
            vec!["Nodes".to_string(), state.nodes_count.to_string()],
            vec!["Edges".to_string(), state.edges_count.to_string()],
        ];
        TableWidget::render(&["Metric", "Value"], &rows);
        if let Some(ref err) = state.last_error {
            println!("Error: {err}");
        }
    }
}
