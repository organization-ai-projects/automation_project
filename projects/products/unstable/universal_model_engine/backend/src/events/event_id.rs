// projects/products/unstable/universal_model_engine/backend/src/events/event_id.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EventId(pub String);
