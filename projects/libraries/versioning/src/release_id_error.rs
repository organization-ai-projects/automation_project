// projects/libraries/versioning/src/release_id_error.rs

#[derive(Debug, thiserror::Error)]
pub enum ReleaseIdError {
    #[error("Invalid format - expected X.Y.Z")]
    InvalidFormat,
    #[error("Invalid number in version")]
    InvalidNumber,
}
