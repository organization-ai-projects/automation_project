// projects/products/unstable/autonomy_orchestrator_ai/src/domain/run_report.rs
use serde::{Deserialize, Serialize};

use crate::domain::{
    AdaptivePolicyDecision, DecisionContribution, DecisionReliabilityFactor,
    DecisionReliabilityUpdate, FinalDecision, GateDecision, PlannerPathRecord, Stage,
    StageExecutionRecord, StageTransition, TerminalState,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunReport {
    pub product: String,
    pub version: String,
    pub run_id: String,
    pub current_stage: Option<Stage>,
    pub terminal_state: Option<TerminalState>,
    pub gate_decisions: Vec<GateDecision>,
    pub blocked_reason_codes: Vec<String>,
    pub reviewer_next_steps: Vec<String>,
    pub final_decision: Option<FinalDecision>,
    pub decision_confidence: Option<u8>,
    pub decision_rationale_codes: Vec<String>,
    pub decision_contributions: Vec<DecisionContribution>,
    pub decision_threshold: Option<u8>,
    pub decision_reliability_factors: Vec<DecisionReliabilityFactor>,
    pub decision_reliability_updates: Vec<DecisionReliabilityUpdate>,
    pub adaptive_policy_decisions: Vec<AdaptivePolicyDecision>,
    pub transitions: Vec<StageTransition>,
    pub stage_executions: Vec<StageExecutionRecord>,
    pub planner_path_record: Option<PlannerPathRecord>,
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
            blocked_reason_codes: Vec::new(),
            reviewer_next_steps: Vec::new(),
            final_decision: None,
            decision_confidence: None,
            decision_rationale_codes: Vec::new(),
            decision_contributions: Vec::new(),
            decision_threshold: None,
            decision_reliability_factors: Vec::new(),
            decision_reliability_updates: Vec::new(),
            adaptive_policy_decisions: Vec::new(),
            transitions: Vec::new(),
            stage_executions: Vec::new(),
            planner_path_record: None,
        }
    }
}
