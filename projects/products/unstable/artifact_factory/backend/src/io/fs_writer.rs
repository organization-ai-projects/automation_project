use crate::bundle::artifact_bundle::ArtifactBundle;
use crate::diagnostics::error::FactoryError;
use std::path::Path;

pub struct FsWriter;

impl FsWriter {
    /// Write all bundle files to `output_dir`. Creates the directory if needed.
    pub fn write_bundle(bundle: &ArtifactBundle, output_dir: &Path) -> Result<(), FactoryError> {
        std::fs::create_dir_all(output_dir).map_err(|e| FactoryError::Io(e.to_string()))?;
        for name in &bundle.manifest {
            if let Some(bytes) = bundle.files.get(name) {
                let file_path = output_dir.join(name);
                std::fs::write(&file_path, bytes).map_err(|e| FactoryError::Io(e.to_string()))?;
            }
        }
        Ok(())
    }
}
