use thiserror::Error;

#[derive(Error, Debug)]
pub enum RendererError {
    #[error("Renderer not initialized")]
    NotInitialized,
    #[error("Unsupported operation")]
    Unsupported,
}
