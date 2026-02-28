// projects/products/unstable/code_forge_engine/backend/src/generate/file_generator.rs
use crate::contract::file_spec::FileSpec;
use crate::diagnostics::error::ForgeError;

pub struct FileGenerator {
    spec: FileSpec,
}

impl FileGenerator {
    pub fn new(spec: FileSpec) -> Self {
        Self { spec }
    }

    pub fn generate_bytes(&self) -> Result<Vec<u8>, ForgeError> {
        Ok(self.spec.content_template.as_bytes().to_vec())
    }
}
