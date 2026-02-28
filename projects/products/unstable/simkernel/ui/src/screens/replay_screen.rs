#![allow(dead_code)]
pub struct ReplayScreen {
    pub replay_path: String,
}
impl ReplayScreen {
    pub fn render(&self) -> String {
        format!("Replay: {}", self.replay_path)
    }
}
