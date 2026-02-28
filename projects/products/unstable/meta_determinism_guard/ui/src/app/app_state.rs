#[derive(Debug, Default)]
pub struct AppState {
    pub scan_findings: Vec<String>,
    pub canon_issues: Vec<String>,
    pub last_response: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }
}
