// projects/products/unstable/simkernel/ui/src/screens/report_screen.rs
pub struct ReportScreen {
    pub report_json: String,
}
impl ReportScreen {
    pub fn render(&self) -> String {
        format!(
            "Report: {}",
            &self.report_json[..self.report_json.len().min(80)]
        )
    }
}
