// projects/products/unstable/digital_pet/ui/src/widgets/log_widget.rs

pub struct LogWidget {
    pub entries: Vec<String>,
}

impl LogWidget {
    pub fn new(entries: Vec<String>) -> Self {
        Self { entries }
    }

    pub fn render(&self) {
        println!("=== Event Log ===");
        for entry in &self.entries {
            println!("  {}", entry);
        }
    }
}
