#[derive(Debug, Clone)]
pub enum Action {
    LoadContract(String),
    Validate,
    Preview,
    Generate { out_dir: String, mode: String },
    GetManifest,
    Shutdown,
}
