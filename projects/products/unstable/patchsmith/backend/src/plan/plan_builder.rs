use sha2::{Digest, Sha256};

use super::patch_plan::PatchPlan;
use crate::diagnostics::error::PatchsmithError;
use crate::dsl::dsl_op::DslOp;

pub struct PlanBuilder;

impl PlanBuilder {
    /// Build a deterministic plan from ops. The plan_hash is SHA-256 of the canonical JSON.
    pub fn build(ops: Vec<DslOp>) -> Result<PatchPlan, PatchsmithError> {
        if ops.is_empty() {
            return Err(PatchsmithError::Parse(
                "cannot build plan from empty ops".into(),
            ));
        }
        let canonical = common_json::to_string(&ops)
            .map_err(|e| PatchsmithError::Internal(format!("serialization error: {e}")))?;
        let mut hasher = Sha256::new();
        hasher.update(canonical.as_bytes());
        let plan_hash = hex::encode(hasher.finalize());
        Ok(PatchPlan { ops, plan_hash })
    }
}
