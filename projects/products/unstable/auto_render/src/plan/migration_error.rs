use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum MigrationError {
    #[error("Requires migration from {from} to {to}")]
    RequiresMigration { from: String, to: String },
    #[error("Incompatible schema versions")]
    Incompatible,
}
