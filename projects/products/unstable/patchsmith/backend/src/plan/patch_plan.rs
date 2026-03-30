use serde::{Deserialize, Serialize};

use crate::dsl::dsl_op::DslOp;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchPlan {
    pub ops: Vec<DslOp>,
    pub plan_hash: String,
}
