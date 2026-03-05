#[derive(Debug, Clone)]
pub struct AppState {
    pub backend_bin: String,
    pub last_command: Option<String>,
    pub last_exit_code: Option<i32>,
    pub last_error: Option<String>,
}

impl AppState {
    pub fn new(backend_bin: String) -> Self {
        Self {
            backend_bin,
            last_command: None,
            last_exit_code: None,
            last_error: None,
        }
    }
}
