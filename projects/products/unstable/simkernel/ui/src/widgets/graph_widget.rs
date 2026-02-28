#![allow(dead_code)]
pub struct GraphWidget { pub values: Vec<i64>, pub label: String }
impl GraphWidget {
    pub fn render(&self) -> String { format!("{}: {:?}", self.label, self.values) }
}
