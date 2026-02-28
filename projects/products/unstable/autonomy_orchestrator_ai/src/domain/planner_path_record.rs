// projects/products/unstable/autonomy_orchestrator_ai/src/domain/planner_path_record.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannerPathRecord {
    pub selected_path: Vec<String>,
    pub fallback_steps_used: u32,
    pub reason_codes: Vec<String>,
}
