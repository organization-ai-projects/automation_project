use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EventType {
    JobLoss,
    NewJob,
    HealthIssue,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventMetadata {
    pub reason: Option<String>,
    pub additional_data: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifeEvent {
    pub event_type: EventType,
    pub date: String,
    pub metadata: EventMetadata,
}
