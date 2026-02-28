// projects/products/unstable/protocol_builder/ui/src/screens/generate_screen.rs

#[derive(Debug, Clone, Default)]
pub struct GenerateScreen {
    pub out_dir: String,
}

impl GenerateScreen {
    pub fn render(&self) -> String {
        format!("Generate -> {}", self.out_dir)
    }
}
