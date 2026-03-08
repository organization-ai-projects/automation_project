#[derive(Debug, Clone)]
pub struct MigrationScreen {
    pub summary: String,
}

impl MigrationScreen {
    pub fn render(&self) -> String {
        format!("[Migration] {}", self.summary)
    }
}
