// projects/products/unstable/spec_to_runtime_compiler/ui/src/widgets/table_widget.rs
use crate::diagnostics::error::UiError;

pub struct TableWidget {
    pub label: String,
}

impl TableWidget {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }

    pub fn render(&self, headers: &[&str], rows: &[Vec<String>]) -> Result<(), UiError> {
        if headers.is_empty() {
            return Err(UiError::Render(
                "table render skipped because headers are empty".to_string(),
            ));
        }
        tracing::debug!(
            label = %self.label,
            columns = headers.len(),
            rows = rows.len(),
            "TableWidget rendered"
        );
        Ok(())
    }
}
