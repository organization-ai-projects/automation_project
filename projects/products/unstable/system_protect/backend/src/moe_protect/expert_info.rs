use serde::{Deserialize, Serialize};

use super::expert_id::ExpertId;
use super::expert_type::ExpertType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpertInfo {
    pub id: ExpertId,
    pub name: String,
    pub expert_type: ExpertType,
}
