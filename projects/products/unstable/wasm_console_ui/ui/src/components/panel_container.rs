/// Describes a panel container for rendering.
/// Holds the active panel ID and list of panel titles.
pub struct PanelContainer {
    pub active_panel_id: Option<String>,
    pub panel_titles: Vec<String>,
}

impl PanelContainer {
    pub fn new(active_panel_id: Option<String>, panel_titles: Vec<String>) -> Self {
        Self {
            active_panel_id,
            panel_titles,
        }
    }

    pub fn has_active_panel(&self) -> bool {
        self.active_panel_id.is_some()
    }

    pub fn panel_count(&self) -> usize {
        self.panel_titles.len()
    }
}
