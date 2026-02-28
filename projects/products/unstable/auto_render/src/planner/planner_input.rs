use serde::{Deserialize, Serialize};
use crate::intent::Intent;
use crate::policy::PolicySnapshot;
use super::WorldSnapshot;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannerInput {
    pub intent: Intent,
    pub policy_snapshot: PolicySnapshot,
    pub world_snapshot: WorldSnapshot,
}
