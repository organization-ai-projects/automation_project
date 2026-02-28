#![allow(dead_code)]
pub struct PackScreen { pub packs: Vec<String> }
impl PackScreen {
    pub fn render(&self) -> String { format!("Packs: {}", self.packs.join(", ")) }
}
