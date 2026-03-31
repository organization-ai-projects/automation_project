use serde::{Deserialize, Serialize};

use super::threat_id::ThreatId;
use super::threat_level::ThreatLevel;
use super::threat_type::ThreatType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatEvent {
    pub id: ThreatId,
    pub threat_type: ThreatType,
    pub threat_level: ThreatLevel,
    pub source: String,
    pub target: String,
    pub payload: String,
    pub timestamp: u64,
}

impl ThreatEvent {
    #[allow(dead_code)]
    pub fn new(
        threat_type: ThreatType,
        threat_level: ThreatLevel,
        source: impl Into<String>,
        target: impl Into<String>,
        payload: impl Into<String>,
    ) -> Self {
        Self {
            id: ThreatId::new(),
            threat_type,
            threat_level,
            source: source.into(),
            target: target.into(),
            payload: payload.into(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    #[allow(dead_code)]
    pub fn with_id(mut self, id: ThreatId) -> Self {
        self.id = id;
        self
    }
}
