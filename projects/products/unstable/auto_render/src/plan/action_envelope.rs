use super::{ActionParameters, ActionType, Capability, Postcondition, Precondition};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionEnvelope {
    pub action_id: String,
    pub action_type: ActionType,
    pub capability_required: Capability,
    pub parameters: ActionParameters,
    pub preconditions: Vec<Precondition>,
    pub postconditions: Vec<Postcondition>,
}
