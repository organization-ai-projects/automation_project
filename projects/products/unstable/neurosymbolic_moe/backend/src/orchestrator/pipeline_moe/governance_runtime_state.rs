//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/pipeline_moe/governance_runtime_state.rs
use crate::evaluations::EvaluationEngine;
use crate::moe_core::MoeError;
use crate::orchestrator::{
    ContinuousGovernancePolicy, ContinuousImprovementReport, GovernanceAuditEntry,
    GovernanceImportPolicy, GovernanceStateSnapshot, Version,
};

#[derive(Clone)]
pub(in crate::orchestrator) struct GovernanceRuntimeState {
    pub continuous_governance_policy: Option<ContinuousGovernancePolicy>,
    pub governance_import_policy: GovernanceImportPolicy,
    pub evaluation_baseline: Option<EvaluationEngine>,
    pub last_continuous_improvement_report: Option<ContinuousImprovementReport>,
    pub governance_state_version: Version,
    pub governance_audit_entries: Vec<GovernanceAuditEntry>,
    pub max_governance_audit_entries: usize,
    pub governance_state_snapshots: Vec<GovernanceStateSnapshot>,
    pub max_governance_state_snapshots: usize,
}

impl GovernanceRuntimeState {
    pub fn validate_invariants(&self) -> Result<(), MoeError> {
        if self.governance_audit_entries.len() > self.max_governance_audit_entries {
            return Err(MoeError::PolicyRejected(format!(
                "governance invariant failed: audit entries exceed max ({} > {})",
                self.governance_audit_entries.len(),
                self.max_governance_audit_entries
            )));
        }
        if self.governance_state_snapshots.len() > self.max_governance_state_snapshots {
            return Err(MoeError::PolicyRejected(format!(
                "governance invariant failed: snapshots exceed max ({} > {})",
                self.governance_state_snapshots.len(),
                self.max_governance_state_snapshots
            )));
        }

        let mut last_version = Version::default();
        for (idx, entry) in self.governance_audit_entries.iter().enumerate() {
            if idx > 0 && entry.version <= last_version {
                return Err(MoeError::PolicyRejected(
                    "governance invariant failed: audit versions are not strictly increasing"
                        .to_string(),
                ));
            }
            last_version = entry.version.clone();
        }

        if let Some(last_audit) = self.governance_audit_entries.last()
            && self.governance_state_version < last_audit.version
        {
            return Err(MoeError::PolicyRejected(format!(
                "governance invariant failed: state version {} below latest audit version {}",
                self.governance_state_version, last_audit.version
            )));
        }

        for snapshot in &self.governance_state_snapshots {
            if snapshot.version != snapshot.state.version_number {
                return Err(MoeError::PolicyRejected(format!(
                    "governance invariant failed: snapshot version {} mismatches state version {}",
                    snapshot.version, snapshot.state.version_number
                )));
            }
        }

        Ok(())
    }
}
