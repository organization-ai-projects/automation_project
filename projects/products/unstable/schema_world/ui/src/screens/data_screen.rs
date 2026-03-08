#[derive(Debug, Clone)]
pub struct DataScreen {
    pub summary: String,
}

impl DataScreen {
    pub fn render(&self) -> String {
        format!("[Data] {}", self.summary)
    }
}
