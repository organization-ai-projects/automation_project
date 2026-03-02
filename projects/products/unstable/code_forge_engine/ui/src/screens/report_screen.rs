// projects/products/unstable/code_forge_engine/ui/src/screens/report_screen.rs
pub struct ReportScreen {
    pub manifest_hash: Option<String>,
    pub file_count: usize,
}

impl ReportScreen {
    pub fn new() -> Self {
        Self {
            manifest_hash: None,
            file_count: 0,
        }
    }
}
