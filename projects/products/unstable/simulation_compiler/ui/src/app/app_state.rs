// projects/products/unstable/simulation_compiler/ui/src/app/app_state.rs

#[derive(Debug, Default)]
pub struct AppState {
    pub dsl_path: String,
    pub dsl_source: Option<String>,
    pub last_report: Option<String>,
    pub error: Option<String>,
}
