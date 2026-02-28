use serde::{Deserialize, Serialize};
use super::{PlanId, PlanSchemaVersion};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanMetadata {
    pub plan_id: PlanId,
    pub plan_schema_version: PlanSchemaVersion,
    pub engine_version: String,
    pub planner_id: String,
    pub planner_version: String,
    pub policy_snapshot_id: String,
    pub seed: u64,
    pub inputs_hash: String,
    pub created_at: String,
    pub explain: String,
    pub explain_trace_ref: Option<String>,
}
