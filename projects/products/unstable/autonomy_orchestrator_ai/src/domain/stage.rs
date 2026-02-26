// projects/products/unstable/autonomy_orchestrator_ai/src/domain/stage.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Stage {
    Planning,
    PolicyIngestion,
    Execution,
    Validation,
    Closure,
}
