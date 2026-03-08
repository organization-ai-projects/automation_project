#[derive(Debug, Clone)]
pub struct DiffScreen {
    pub summary: String,
}

impl DiffScreen {
    pub fn render(&self) -> String {
        format!("[Diff] {}", self.summary)
    }
}
