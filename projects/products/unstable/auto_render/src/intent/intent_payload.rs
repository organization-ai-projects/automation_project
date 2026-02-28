use serde::{Deserialize, Serialize};
use super::CinematographyPayload;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntentPayload {
    Cinematography(CinematographyPayload),
}
