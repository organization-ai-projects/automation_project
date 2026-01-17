// projects/libraries/protocol/src/log_level.rs
use serde::{Deserialize, Serialize};

/// Remplacement de log_level par une définition inline temporaire
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}
