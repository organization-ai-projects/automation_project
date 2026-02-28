use super::{AssetError, AssetGenerator};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

pub struct FileAssetGenerator {
    output_dir: PathBuf,
}

impl FileAssetGenerator {
    pub fn new(output_dir: PathBuf) -> Self {
        Self { output_dir }
    }
}

impl AssetGenerator for FileAssetGenerator {
    fn generate(&self, spec: &str) -> Result<(), AssetError> {
        if spec.trim().is_empty() {
            return Err(AssetError::Unsupported);
        }

        std::fs::create_dir_all(&self.output_dir).map_err(|_| AssetError::IoNotPermitted)?;

        let mut hasher = Sha256::new();
        hasher.update(spec.as_bytes());
        let digest = hex::encode(hasher.finalize());
        let output_path = self.output_dir.join(format!("{digest}.asset.txt"));

        std::fs::write(output_path, spec).map_err(|_| AssetError::GenerationDenied)
    }
}
