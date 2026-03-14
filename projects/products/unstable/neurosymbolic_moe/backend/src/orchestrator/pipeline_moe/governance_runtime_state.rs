//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/pipeline_moe/governance_runtime_state.rs
use crate::evaluation_engine::EvaluationEngine;
use crate::orchestrator::{
    ContinuousGovernancePolicy, ContinuousImprovementReport, GovernanceAuditEntry,
    GovernanceImportPolicy, GovernanceStateSnapshot,
};

#[derive(Clone)]
pub(in crate::orchestrator) struct GovernanceRuntimeState {
    pub continuous_governance_policy: Option<ContinuousGovernancePolicy>,
    pub governance_import_policy: GovernanceImportPolicy,
    pub evaluation_baseline: Option<EvaluationEngine>,
    pub last_continuous_improvement_report: Option<ContinuousImprovementReport>,
    pub governance_state_version: u64,
    pub governance_audit_entries: Vec<GovernanceAuditEntry>,
    pub max_governance_audit_entries: usize,
    pub governance_state_snapshots: Vec<GovernanceStateSnapshot>,
    pub max_governance_state_snapshots: usize,
}
