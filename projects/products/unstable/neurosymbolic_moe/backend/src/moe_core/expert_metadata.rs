use serde::{Deserialize, Serialize};

use crate::orchestrator::Version;

use super::expert_capability::ExpertCapability;
use super::expert_id::ExpertId;
use super::expert_status::ExpertStatus;
use super::expert_type::ExpertType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpertMetadata {
    pub id: ExpertId,
    pub name: String,
    pub version: Version,
    pub capabilities: Vec<ExpertCapability>,
    pub status: ExpertStatus,
    pub expert_type: ExpertType,
}
