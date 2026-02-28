// projects/products/unstable/autonomy_orchestrator_ai/src/domain/run_report.rs
use serde::{Deserialize, Serialize};

use crate::domain::{
    AdaptivePolicyDecision, AutoFixAttempt, DecisionContribution, DecisionReliabilityFactor,
    DecisionReliabilityUpdate, EscalationCase, FinalDecision, GateDecision, HardGateResult,
    PlannerPathRecord, PrRiskBreakdown, ProvenanceRecord, ReviewEnsembleResult, ReviewerVerdict,
    RiskSignal, RiskTier, RollbackDecision, RolloutStep, Stage, StageExecutionRecord,
    StageTransition, TerminalState,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunReport {
    pub product: String,
    pub version: String,
    pub run_id: String,
    pub current_stage: Option<Stage>,
    pub terminal_state: Option<TerminalState>,
    pub gate_decisions: Vec<GateDecision>,
    pub hard_gate_results: Vec<HardGateResult>,
    pub blocked_reason_codes: Vec<String>,
    pub reviewer_next_steps: Vec<String>,
    pub reviewer_verdicts: Vec<ReviewerVerdict>,
    pub review_ensemble_result: Option<ReviewEnsembleResult>,
    pub final_decision: Option<FinalDecision>,
    pub decision_confidence: Option<u8>,
    pub decision_rationale_codes: Vec<String>,
    pub decision_contributions: Vec<DecisionContribution>,
    pub decision_threshold: Option<u8>,
    pub decision_reliability_factors: Vec<DecisionReliabilityFactor>,
    pub decision_reliability_updates: Vec<DecisionReliabilityUpdate>,
    pub adaptive_policy_decisions: Vec<AdaptivePolicyDecision>,
    pub auto_fix_attempts: Vec<AutoFixAttempt>,
    pub transitions: Vec<StageTransition>,
    pub stage_executions: Vec<StageExecutionRecord>,
    pub provenance_records: Vec<ProvenanceRecord>,
    pub provenance_schema_version: String,
    pub escalation_cases: Vec<EscalationCase>,
    pub rollout_steps: Vec<RolloutStep>,
    pub rollback_decision: Option<RollbackDecision>,
    pub pr_risk_breakdown: Option<PrRiskBreakdown>,
    pub planner_path_record: Option<PlannerPathRecord>,
    pub risk_tier: Option<RiskTier>,
    pub risk_signals: Vec<RiskSignal>,
}

impl RunReport {
    pub fn new(run_id: String) -> Self {
        Self {
            product: "autonomy_orchestrator_ai".to_string(),
            version: "0.1.0".to_string(),
            run_id,
            current_stage: None,
            terminal_state: None,
            gate_decisions: Vec::new(),
            hard_gate_results: Vec::new(),
            blocked_reason_codes: Vec::new(),
            reviewer_next_steps: Vec::new(),
            reviewer_verdicts: Vec::new(),
            review_ensemble_result: None,
            final_decision: None,
            decision_confidence: None,
            decision_rationale_codes: Vec::new(),
            decision_contributions: Vec::new(),
            decision_threshold: None,
            decision_reliability_factors: Vec::new(),
            decision_reliability_updates: Vec::new(),
            adaptive_policy_decisions: Vec::new(),
            auto_fix_attempts: Vec::new(),
            transitions: Vec::new(),
            stage_executions: Vec::new(),
            provenance_records: Vec::new(),
            provenance_schema_version: "1".to_string(),
            escalation_cases: Vec::new(),
            rollout_steps: Vec::new(),
            rollback_decision: None,
            pr_risk_breakdown: None,
            planner_path_record: None,
            risk_tier: None,
            risk_signals: Vec::new(),
        }
    }
}
