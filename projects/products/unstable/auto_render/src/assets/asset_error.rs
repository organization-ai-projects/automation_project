use thiserror::Error;

#[derive(Error, Debug)]
pub enum AssetError {
    #[error("Asset generation denied")]
    GenerationDenied,
    #[error("IO not permitted")]
    IoNotPermitted,
    #[error("Unsupported asset type")]
    Unsupported,
}
