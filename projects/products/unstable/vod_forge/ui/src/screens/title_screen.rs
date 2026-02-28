use crate::app::AppState;

pub struct TitleScreen;

impl TitleScreen {
    pub fn render(state: &AppState, title_id: &str) -> String {
        let mut out = format!("=== Title: {} ===\n", title_id);
        if let Some(t) = state.catalog_titles.iter().find(|t| t.id == title_id) {
            out.push_str(&format!("Name: {}\nYear: {}\n", t.name, t.year));
        } else {
            out.push_str("(not found)\n");
        }
        out
    }
}
