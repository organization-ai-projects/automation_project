// projects/products/unstable/code_forge_engine/ui/src/screens/preview_screen.rs
pub struct PreviewScreen {
    pub files: Vec<String>,
}

impl PreviewScreen {
    pub fn new() -> Self {
        Self { files: vec![] }
    }

    pub fn set_files(&mut self, files: Vec<String>) {
        self.files = files;
    }
}
