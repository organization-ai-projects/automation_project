// projects/products/unstable/digital_pet/ui/src/screens/training_screen.rs

pub struct TrainingScreen {
    pub result: String,
}

impl TrainingScreen {
    pub fn new(result: String) -> Self {
        Self { result }
    }

    pub fn render(&self) {
        println!("=== Training ===");
        println!("  {}", self.result);
    }
}
