// projects/products/unstable/digital_pet/ui/src/screens/evolution_screen.rs

pub struct EvolutionScreen {
    pub from: String,
    pub to: String,
    pub stage: u32,
}

impl EvolutionScreen {
    pub fn render(&self) {
        println!("=== Evolution! ===");
        println!("  {} -> {} (stage {})", self.from, self.to, self.stage);
    }
}
