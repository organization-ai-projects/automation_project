#[derive(Debug, Clone)]
pub struct SchemaScreen {
    pub summary: String,
}

impl SchemaScreen {
    pub fn render(&self) -> String {
        format!("[Schema] {}", self.summary)
    }
}
