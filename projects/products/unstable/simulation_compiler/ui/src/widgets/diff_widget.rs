// projects/products/unstable/simulation_compiler/ui/src/widgets/diff_widget.rs

pub struct DiffWidget {
    pub label: String,
}

impl DiffWidget {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }

    pub fn render(&self, before: &str, after: &str) {
        tracing::debug!(label = %self.label, before_len = before.len(), after_len = after.len(), "DiffWidget rendered");
    }
}
