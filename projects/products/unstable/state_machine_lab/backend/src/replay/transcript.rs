use crate::execute::step_result::StepResult;
use crate::model::machine_id::MachineId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transcript {
    pub machine_id: MachineId,
    pub seed: Option<u64>,
    pub steps: Vec<StepResult>,
    pub final_state: String,
}
