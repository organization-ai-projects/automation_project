use crate::events::event_id::EventId;
use crate::transitions::transition::Transition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelEvent {
    pub id: EventId,
    pub step: u64,
    pub transition_id: String,
    pub target_var: String,
    pub delta: i64,
    pub value_after: i64,
    pub labels: Vec<String>,
}

impl ModelEvent {
    pub fn from_transition(
        step: u64,
        transition: &Transition,
        value_after: i64,
        labels: Vec<String>,
    ) -> Self {
        Self {
            id: EventId(format!("ev{}", step)),
            step,
            transition_id: transition.transition_label(),
            target_var: transition.target_var.0.clone(),
            delta: transition.delta,
            value_after,
            labels,
        }
    }
}
