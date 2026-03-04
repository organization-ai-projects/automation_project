use crate::diagnostics::backend_error::BackendError;
use std::collections::BTreeMap;

pub struct TemplateEngine {
    variables: BTreeMap<String, String>,
}

impl TemplateEngine {
    pub fn new() -> Self {
        Self {
            variables: BTreeMap::new(),
        }
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.variables.insert(key.into(), value.into());
    }

    pub fn render(&self, template: &str) -> Result<String, BackendError> {
        let mut rendered = template.to_string();
        for (key, value) in &self.variables {
            rendered = rendered.replace(&format!("{{{{{key}}}}}"), value);
        }
        Ok(rendered)
    }
}
