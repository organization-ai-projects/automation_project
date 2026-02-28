// projects/products/unstable/code_forge_engine/backend/src/generate/template_engine.rs
use std::collections::BTreeMap;
use crate::diagnostics::error::ForgeError;

pub struct TemplateEngine {
    variables: BTreeMap<String, String>,
}

impl TemplateEngine {
    pub fn new() -> Self {
        Self { variables: BTreeMap::new() }
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.variables.insert(key.into(), value.into());
    }

    pub fn render(&self, template: &str) -> Result<String, ForgeError> {
        let mut out = template.to_string();
        for (k, v) in &self.variables {
            out = out.replace(&format!("{{{{{k}}}}}"), v);
        }
        Ok(out)
    }
}
