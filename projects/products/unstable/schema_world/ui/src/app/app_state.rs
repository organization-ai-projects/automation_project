use common_json::Json;

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub schema: Option<Json>,
    pub snapshot_hash: Option<String>,
    pub report_json: Option<Json>,
    pub rendered_view: Option<String>,
    pub last_error: Option<String>,
}
