// projects/products/unstable/autonomous_dev_ai/src/pr_flow/merge_readiness_checker.rs
use super::{CiStatus, IssueComplianceStatus, MergeReadiness, PrMetadata};
use serde::{Deserialize, Serialize};

/// Aggregates CI status, policy status, and issue compliance to determine
/// whether a PR is ready to merge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeReadinessChecker;

impl MergeReadinessChecker {
    pub fn check(pr: &PrMetadata) -> MergeReadiness {
        let mut reasons = Vec::new();

        if pr.ci_status != CiStatus::Passing {
            reasons.push(format!("CI is not passing (status: {:?})", pr.ci_status));
        }

        if !pr.policy_compliant {
            reasons.push("Policy compliance check has not passed".to_string());
        }

        match &pr.issue_compliance {
            IssueComplianceStatus::Compliant => {}
            IssueComplianceStatus::NonCompliant { reason } => {
                reasons.push(format!("Issue compliance failed: {reason}"));
            }
            IssueComplianceStatus::Unknown => {
                reasons.push("Issue compliance status is unknown".to_string());
            }
        }

        if reasons.is_empty() {
            MergeReadiness::Ready
        } else {
            MergeReadiness::NotReady { reasons }
        }
    }
}
