use crate::app::AppState;
use crate::widgets::TableWidget;

pub struct CatalogScreen;

impl CatalogScreen {
    pub fn render(state: &AppState) -> String {
        let mut out = String::from("=== Catalog ===\n");
        if state.catalog_titles.is_empty() {
            out.push_str("(no titles)\n");
        } else {
            let rows: Vec<Vec<String>> = state
                .catalog_titles
                .iter()
                .map(|t| vec![t.id.clone(), t.name.clone(), t.year.to_string()])
                .collect();
            out.push_str(&TableWidget::render(&["ID", "Name", "Year"], &rows));
        }
        out
    }
}
