#![allow(dead_code)]
pub struct RunScreen {
    pub tick: u64,
    pub pack_kind: String,
}
impl RunScreen {
    pub fn render(&self) -> String {
        format!("Running {} - tick {}", self.pack_kind, self.tick)
    }
}
