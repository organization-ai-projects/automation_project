use crate::app::app_state::AppState;
use crate::widgets::table_widget::TableWidget;

pub struct BundleScreen;

impl BundleScreen {
    pub fn render(state: &AppState) {
        println!("=== Bundle Screen ===");
        if let Some(ref hash) = state.bundle_hash {
            println!("Bundle hash: {hash}");
            println!("Manifest:");
            let rows: Vec<Vec<String>> = state
                .bundle_manifest
                .iter()
                .enumerate()
                .map(|(index, entry)| vec![(index + 1).to_string(), entry.clone()])
                .collect();
            TableWidget::render(&["#", "File"], &rows);
        } else {
            println!("No bundle available.");
        }
    }
}
