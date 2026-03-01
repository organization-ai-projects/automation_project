#![allow(dead_code)]

/// Renders a simple ASCII bar graph.
pub struct GraphWidget {
    pub title: String,
    pub values: Vec<(String, u64)>,
}

impl GraphWidget {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            values: Vec::new(),
        }
    }

    pub fn add_bar(&mut self, label: impl Into<String>, value: u64) {
        self.values.push((label.into(), value));
    }

    pub fn render(&self) -> String {
        let max = self.values.iter().map(|(_, v)| *v).max().unwrap_or(1).max(1);
        let mut out = format!("=== {} ===\n", self.title);
        for (label, value) in &self.values {
            let bar_len = (value * 20 / max) as usize;
            let bar = "#".repeat(bar_len);
            out.push_str(&format!("{:>10}: {} ({})\n", label, bar, value));
        }
        out
    }
}
