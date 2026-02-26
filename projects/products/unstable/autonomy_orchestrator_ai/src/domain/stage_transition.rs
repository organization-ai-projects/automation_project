// projects/products/unstable/autonomy_orchestrator_ai/src/domain/stage_transition.rs
use serde::{Deserialize, Serialize};

use crate::domain::Stage;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageTransition {
    pub run_id: String,
    pub from_stage: Option<Stage>,
    pub to_stage: Stage,
    pub timestamp_unix_secs: u64,
}
