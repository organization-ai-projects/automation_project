#[derive(Debug, Clone)]
pub struct ReportScreen {
    pub summary: String,
}

impl ReportScreen {
    pub fn render(&self) -> String {
        format!("[Report] {}", self.summary)
    }
}
