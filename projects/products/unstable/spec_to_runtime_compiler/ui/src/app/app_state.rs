// projects/products/unstable/spec_to_runtime_compiler/ui/src/app/app_state.rs

#[derive(Debug, Default)]
pub struct AppState {
    pub spec_path: String,
    pub spec_source: Option<String>,
    pub last_report: Option<String>,
    pub error: Option<String>,
}
