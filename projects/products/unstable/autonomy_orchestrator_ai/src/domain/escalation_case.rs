// projects/products/unstable/autonomy_orchestrator_ai/src/domain/escalation_case.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EscalationSeverity {
    Sev1,
    Sev2,
    Sev3,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EscalationCase {
    pub id: String,
    pub trigger_code: String,
    pub severity: EscalationSeverity,
    pub required_actions: Vec<String>,
    pub context_artifacts: Vec<String>,
}
