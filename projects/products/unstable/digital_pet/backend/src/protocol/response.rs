// projects/products/unstable/digital_pet/backend/src/protocol/response.rs
use crate::battle::battle_state::BattleState;
use crate::model::pet_state::PetState;
use crate::report::run_report::RunReport;
use crate::snapshot::state_snapshot::StateSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Response {
    Ok { id: Option<u64> },
    OkWithData { id: Option<u64>, data: String },
    Error { id: Option<u64>, message: String },
    PetState { id: Option<u64>, state: PetState },
    BattleState { id: Option<u64>, state: BattleState },
    Snapshot { id: Option<u64>, snapshot: StateSnapshot },
    Report { id: Option<u64>, report: RunReport },
}

impl Response {
    pub fn ok(id: Option<u64>) -> Self { Self::Ok { id } }
    pub fn ok_with_data(id: Option<u64>, data: String) -> Self { Self::OkWithData { id, data } }
    pub fn error(id: Option<u64>, message: &str) -> Self { Self::Error { id, message: message.to_string() } }
    pub fn pet_state(id: Option<u64>, state: PetState) -> Self { Self::PetState { id, state } }
    pub fn battle_state(id: Option<u64>, state: BattleState) -> Self { Self::BattleState { id, state } }
    pub fn snapshot(id: Option<u64>, snapshot: StateSnapshot) -> Self { Self::Snapshot { id, snapshot } }
    pub fn report(id: Option<u64>, report: RunReport) -> Self { Self::Report { id, report } }
}
