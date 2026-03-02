// projects/products/unstable/code_forge_engine/ui/src/app/action.rs
#[derive(Debug, Clone)]
pub enum Action {
    LoadContract(String),
    Validate,
    Preview,
    Generate { out_dir: String, mode: String },
    GetManifest,
    Shutdown,
}
