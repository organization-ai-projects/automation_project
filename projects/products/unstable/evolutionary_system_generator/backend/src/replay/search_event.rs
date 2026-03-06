// projects/products/unstable/evolutionary_system_generator/backend/src/replay/search_event.rs
use crate::replay::search_event_kind::SearchEventKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEvent {
    pub sequence: u64,
    pub kind: SearchEventKind,
}
