#![allow(dead_code)]
pub struct LogWidget {
    pub entries: Vec<String>,
}
impl LogWidget {
    pub fn render(&self) -> String {
        self.entries.join("\n")
    }
}
