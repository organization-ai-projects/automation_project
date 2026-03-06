use crate::app::screen::Screen;

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub screen: Screen,
    pub contract_path: Option<String>,
    pub preview_files: Vec<String>,
    pub manifest_json: Option<String>,
    pub manifest_hash: Option<String>,
    pub last_error: Option<String>,
}
