#![allow(dead_code)]
pub struct TableWidget {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}
impl TableWidget {
    pub fn render(&self) -> String {
        self.headers.join(" | ")
    }
}
