// projects/products/unstable/hospital_tycoon/ui/src/widgets/chart_widget.rs

pub struct ChartWidget {
    pub label: String,
    pub value: u64,
    pub max: u64,
}

impl ChartWidget {
    pub fn new(label: impl Into<String>, value: u64, max: u64) -> Self {
        Self {
            label: label.into(),
            value,
            max,
        }
    }

    pub fn render(&self) {
        let width = 20usize;
        let filled = if self.max > 0 {
            ((self.value as f64 / self.max as f64) * width as f64) as usize
        } else {
            0
        };
        let bar = "#".repeat(filled) + &"-".repeat(width - filled);
        println!("  {:12}: [{}] {}/{}", self.label, bar, self.value, self.max);
    }
}
