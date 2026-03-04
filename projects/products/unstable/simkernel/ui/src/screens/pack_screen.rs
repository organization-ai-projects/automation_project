// projects/products/unstable/simkernel/ui/src/screens/pack_screen.rs
pub struct PackScreen {
    pub packs: Vec<String>,
}
impl PackScreen {
    pub fn render(&self) -> String {
        format!("Packs: {}", self.packs.join(", "))
    }
}
