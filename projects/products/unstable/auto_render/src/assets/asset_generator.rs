use super::AssetError;

pub trait AssetGenerator {
    fn generate(&self, spec: &str) -> Result<(), AssetError>;
}
