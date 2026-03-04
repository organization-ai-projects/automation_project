use crate::contracts::contract::Contract;
use crate::diagnostics::backend_error::BackendError;
use crate::generate::crate_generator::CrateGenerator;
use crate::generate::file_generator::FileGenerator;
use crate::output::artifact_manifest::ArtifactManifest;

pub struct WorkspaceGenerator {
    contract: Contract,
}

impl WorkspaceGenerator {
    pub fn new(contract: Contract) -> Self {
        Self { contract }
    }

    pub fn generate(&self) -> Result<ArtifactManifest, BackendError> {
        let mut manifest = ArtifactManifest::new(self.contract.name.clone());

        for module in &self.contract.modules {
            let crate_generator = CrateGenerator::new(module.clone());
            let generated_paths = crate_generator.generate_paths();
            for path in generated_paths {
                let file_spec = module
                    .files
                    .iter()
                    .find(|file| file.path == path)
                    .ok_or_else(|| {
                        BackendError::NotFound(format!("file spec not found: {path}"))
                    })?;

                let generator = FileGenerator::new(file_spec.clone());
                let bytes = generator.generate_bytes(&module.name)?;
                manifest.add_file(path, bytes);
            }
        }

        Ok(manifest)
    }
}
