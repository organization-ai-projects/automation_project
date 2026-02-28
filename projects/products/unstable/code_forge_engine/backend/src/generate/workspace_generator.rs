// projects/products/unstable/code_forge_engine/backend/src/generate/workspace_generator.rs
use crate::contract::contract::Contract;
use crate::output::artifact_manifest::ArtifactManifest;
use crate::diagnostics::error::ForgeError;

pub struct WorkspaceGenerator {
    contract: Contract,
}

impl WorkspaceGenerator {
    pub fn new(contract: Contract) -> Self {
        Self { contract }
    }

    pub fn generate(&self) -> Result<ArtifactManifest, ForgeError> {
        let mut manifest = ArtifactManifest::new(self.contract.name.clone());
        for module in &self.contract.modules {
            for file in &module.files {
                manifest.add_file(file.path.clone(), vec![]);
            }
        }
        Ok(manifest)
    }
}
