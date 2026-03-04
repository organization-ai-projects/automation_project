// projects/products/unstable/simulation_compiler/ui/src/widgets/diff_widget.rs
use crate::diagnostics::ui_error::UiError;

pub struct DiffWidget {
    pub label: String,
}

impl DiffWidget {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }

    pub fn render(&self, before: &str, after: &str) -> Result<(), UiError> {
        if before == after {
            return Err(UiError::Render(
                "diff render skipped because inputs are identical".to_string(),
            ));
        }
        tracing::debug!(label = %self.label, before_len = before.len(), after_len = after.len(), "DiffWidget rendered");
        Ok(())
    }
}
