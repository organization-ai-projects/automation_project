use crate::evaluation_engine::EvaluationEngine;
use crate::orchestrator::{ContinuousGovernancePolicy, ContinuousImprovementReport};
use serde::{Deserialize, Serialize};

const GOVERNANCE_STATE_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceState {
    pub schema_version: u32,
    pub state_version: u64,
    pub state_checksum: String,
    pub continuous_governance_policy: Option<ContinuousGovernancePolicy>,
    pub evaluation_baseline: Option<EvaluationEngine>,
    pub last_continuous_improvement_report: Option<ContinuousImprovementReport>,
}

impl GovernanceState {
    pub fn from_components(
        state_version: u64,
        continuous_governance_policy: Option<ContinuousGovernancePolicy>,
        evaluation_baseline: Option<EvaluationEngine>,
        last_continuous_improvement_report: Option<ContinuousImprovementReport>,
    ) -> Self {
        let mut state = Self {
            schema_version: GOVERNANCE_STATE_SCHEMA_VERSION,
            state_version,
            state_checksum: String::new(),
            continuous_governance_policy,
            evaluation_baseline,
            last_continuous_improvement_report,
        };
        state.state_checksum = state.recompute_checksum();
        state
    }

    pub fn recompute_checksum(&self) -> String {
        let policy_fingerprint = self
            .continuous_governance_policy
            .as_ref()
            .map(|p| {
                format!(
                    "{:.6}:{:.6}:{:.6}:{:.6}:{}:{}",
                    p.min_expert_success_rate,
                    p.min_routing_accuracy,
                    p.low_score_threshold,
                    p.regression_drop_threshold,
                    p.block_on_human_review,
                    p.auto_promote_on_pass
                )
            })
            .unwrap_or_else(|| "-".to_string());

        let baseline_fingerprint = self
            .evaluation_baseline
            .as_ref()
            .map(EvaluationEngine::checksum_fingerprint)
            .unwrap_or_else(|| "-".to_string());

        let report_fingerprint = self
            .last_continuous_improvement_report
            .as_ref()
            .map(ContinuousImprovementReport::checksum_fingerprint)
            .unwrap_or_else(|| "-".to_string());

        let material = format!(
            "{}:{}:{}:{}:{}",
            self.schema_version,
            self.state_version,
            policy_fingerprint,
            baseline_fingerprint,
            report_fingerprint
        );

        format!("{:016x}", fnv1a64(material.as_bytes()))
    }

    pub fn verify_checksum(&self) -> bool {
        !self.state_checksum.is_empty() && self.state_checksum == self.recompute_checksum()
    }

    pub fn ensure_checksum(&mut self) {
        if self.state_checksum.is_empty() {
            self.state_checksum = self.recompute_checksum();
        }
    }
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
