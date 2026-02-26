// projects/products/unstable/autonomy_orchestrator_ai/src/domain/gate_inputs.rs

use crate::domain::{CiGateStatus, PolicyGateStatus, ReviewGateStatus};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GateInputs {
    pub policy_status: PolicyGateStatus,
    pub ci_status: CiGateStatus,
    pub review_status: ReviewGateStatus,
}

impl GateInputs {
    #[cfg(test)]
    pub fn passing() -> Self {
        Self {
            policy_status: PolicyGateStatus::Allow,
            ci_status: CiGateStatus::Success,
            review_status: ReviewGateStatus::Approved,
        }
    }
}
