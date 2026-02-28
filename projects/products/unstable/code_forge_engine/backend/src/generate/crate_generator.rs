// projects/products/unstable/code_forge_engine/backend/src/generate/crate_generator.rs
use crate::contract::module_spec::ModuleSpec;
use crate::diagnostics::error::ForgeError;

pub struct CrateGenerator {
    spec: ModuleSpec,
}

impl CrateGenerator {
    pub fn new(spec: ModuleSpec) -> Self {
        Self { spec }
    }

    pub fn generate_paths(&self) -> Result<Vec<String>, ForgeError> {
        Ok(self.spec.files.iter().map(|f| f.path.clone()).collect())
    }
}
