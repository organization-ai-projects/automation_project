use crate::model::event_id::EventId;
use crate::model::state_id::StateId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StepResult {
    pub step: u64,
    pub previous_state: StateId,
    pub event: EventId,
    pub next_state: StateId,
    pub variables: BTreeMap<String, i64>,
}
