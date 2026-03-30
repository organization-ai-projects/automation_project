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
        // Build a deterministic canonical form by hashing each op's fields in a fixed order.
        let mut hasher = Sha256::new();
        for op in &ops {
            let fragment = match op {
                DslOp::ReplaceRange { file, start, end, text } => {
                    format!("ReplaceRange\0{file}\0{start}\0{end}\0{text}\0")
                }
                DslOp::ReplaceFirst { file, pattern, text } => {
                    format!("ReplaceFirst\0{file}\0{pattern}\0{text}\0")
                }
                DslOp::InsertAfter { file, pattern, text } => {
                    format!("InsertAfter\0{file}\0{pattern}\0{text}\0")
                }
                DslOp::DeleteRange { file, start, end } => {
                    format!("DeleteRange\0{file}\0{start}\0{end}\0")
                }
            };
            hasher.update(fragment.as_bytes());
        }
        let plan_hash = hex::encode(hasher.finalize());
        Ok(PatchPlan { ops, plan_hash })
    }
}
