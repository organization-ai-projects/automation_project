#![allow(dead_code)]
pub struct InspectScreen {
    pub query_result: String,
}
impl InspectScreen {
    pub fn render(&self) -> String {
        format!("Inspect: {}", self.query_result)
    }
}
