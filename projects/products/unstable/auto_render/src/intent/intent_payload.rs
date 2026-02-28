use super::CinematographyPayload;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntentPayload {
    Cinematography(CinematographyPayload),
}
