// projects/products/unstable/digital_pet/ui/src/widgets/stat_widget.rs

pub struct StatWidget {
    pub label: String,
    pub value: u32,
    pub max: u32,
}

impl StatWidget {
    pub fn new(label: impl Into<String>, value: u32, max: u32) -> Self {
        Self { label: label.into(), value, max }
    }

    pub fn render(&self) {
        let filled = (self.value as usize * 20) / (self.max.max(1) as usize);
        let bar: String = "#".repeat(filled) + &"-".repeat(20 - filled);
        println!("  {:12}: [{}] {}/{}", self.label, bar, self.value, self.max);
    }
}
