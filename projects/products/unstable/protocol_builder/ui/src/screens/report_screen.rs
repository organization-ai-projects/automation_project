// projects/products/unstable/protocol_builder/ui/src/screens/report_screen.rs

#[derive(Debug, Clone, Default)]
pub struct ReportScreen {
    pub manifest_hash: String,
    pub report_json: String,
}

impl ReportScreen {
    pub fn render(&self) -> String {
        format!("Report hash: {}\n{}", self.manifest_hash, self.report_json)
    }
}
