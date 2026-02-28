// projects/products/unstable/code_forge_engine/ui/src/screens/generate_screen.rs
pub struct GenerateScreen {
    pub out_dir: Option<String>,
    pub mode: String,
}

impl GenerateScreen {
    pub fn new() -> Self {
        Self { out_dir: None, mode: "dry_run".to_string() }
    }
}
