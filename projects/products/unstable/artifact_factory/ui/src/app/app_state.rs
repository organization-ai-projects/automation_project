use crate::app::screen::Screen;

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub current_screen: Screen,
    pub input_paths: Vec<String>,
    pub inputs_total: usize,
    pub input_reports: usize,
    pub input_replays: usize,
    pub input_manifests: usize,
    pub input_protocol_schemas: usize,
    pub input_unknown: usize,
    pub events_count: usize,
    pub protocols_count: usize,
    pub nodes_count: usize,
    pub edges_count: usize,
    pub markdown_bytes: usize,
    pub svg_bytes: usize,
    pub html_bytes: usize,
    pub bundle_hash: Option<String>,
    pub bundle_manifest: Vec<String>,
    pub last_error: Option<String>,
    pub running: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            running: true,
            ..Default::default()
        }
    }
}
