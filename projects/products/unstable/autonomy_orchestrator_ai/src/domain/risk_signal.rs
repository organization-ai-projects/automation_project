// projects/products/unstable/autonomy_orchestrator_ai/src/domain/risk_signal.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskSignal {
    pub code: String,
    pub source: String,
    pub value: String,
}
