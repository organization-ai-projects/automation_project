use crate::app::AppState;

pub struct CatalogScreen;

impl CatalogScreen {
    pub fn render(state: &AppState) -> String {
        let mut out = String::from("=== Catalog ===\n");
        if state.catalog_titles.is_empty() {
            out.push_str("(no titles)\n");
        } else {
            for t in &state.catalog_titles {
                out.push_str(&format!("  [{}] {} ({})\n", t.id, t.name, t.year));
            }
        }
        out
    }
}
