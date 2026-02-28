use super::{AssetGenerator, AssetError};

pub struct NoopAssetGenerator;

impl AssetGenerator for NoopAssetGenerator {
    fn generate(&self, _spec: &str) -> Result<(), AssetError> {
        Ok(())
    }
}
