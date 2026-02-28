// projects/products/unstable/autonomy_orchestrator_ai/src/domain/hard_gate_rule.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HardGateCategory {
    Secrets,
    Auth,
    GitHistory,
    InfraDestructive,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HardGateMode {
    MatchAnyInvocationArg,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardGateRule {
    pub id: String,
    pub category: HardGateCategory,
    pub pattern: String,
    pub mode: HardGateMode,
}
