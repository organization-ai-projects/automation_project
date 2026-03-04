use crate::app::app_state::AppState;
use crate::widgets::table_widget::TableWidget;

pub struct PreviewScreen;

impl PreviewScreen {
    pub fn render(state: &AppState) {
        let rows: Vec<Vec<String>> = state
            .preview_files
            .iter()
            .map(|path| vec![path.clone()])
            .collect();
        TableWidget::render(&["preview files"], &rows);
    }
}
