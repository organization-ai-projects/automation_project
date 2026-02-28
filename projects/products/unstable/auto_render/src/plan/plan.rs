use super::{ActionEnvelope, PlanMetadata};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub metadata: PlanMetadata,
    pub actions: Vec<ActionEnvelope>,
}
