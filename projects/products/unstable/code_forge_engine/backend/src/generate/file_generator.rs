use crate::contract::file_spec::FileSpec;
use crate::diagnostics::backend_error::BackendError;
use crate::generate::template_engine::TemplateEngine;

pub struct FileGenerator {
    spec: FileSpec,
}

impl FileGenerator {
    pub fn new(spec: FileSpec) -> Self {
        Self { spec }
    }

    pub fn generate_bytes(&self, module_name: &str) -> Result<Vec<u8>, BackendError> {
        let mut template = TemplateEngine::new();
        template.set("primary_type", self.spec.primary_type.clone());
        template.set("module_name", module_name.to_string());

        let content = if self.spec.content_template.trim().is_empty() {
            format!(
                "pub struct {} {{\n    pub module: &'static str,\n}}\n",
                self.spec.primary_type
            )
        } else {
            template.render(&self.spec.content_template)?
        };

        Ok(content.into_bytes())
    }
}
