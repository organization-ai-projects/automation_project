use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReplayError {
    #[error("invalid journal: {0}")]
    InvalidJournal(String),

    #[error("journal load error: {0}")]
    LoadError(String),

    #[error("replay mismatch: {0}")]
    Mismatch(String),

    #[error("missing run metadata in journal")]
    MissingMetadata,
}
