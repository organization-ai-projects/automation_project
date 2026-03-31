use serde::{Deserialize, Serialize};

use crate::state::StateValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryEvent {
    pub step: u64,
    pub kind: StoryEventKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StoryEventKind {
    RuleApplied { rule_id: String },
    StateChanged {
        variable: String,
        old_value: StateValue,
        new_value: StateValue,
    },
    Narration { message: String },
    NoApplicableRules,
}
