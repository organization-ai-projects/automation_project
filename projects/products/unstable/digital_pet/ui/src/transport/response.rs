use crate::transport::battle_state_dto::BattleStateDto;
use crate::transport::pet_state_dto::PetStateDto;
use crate::transport::run_report_dto::RunReportDto;
use crate::transport::snapshot_dto::SnapshotDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Response {
    Ok {
        id: Option<u64>,
    },
    Error {
        id: Option<u64>,
        message: String,
    },
    OkWithData {
        id: Option<u64>,
        data: String,
    },
    PetState {
        id: Option<u64>,
        state: PetStateDto,
    },
    BattleState {
        id: Option<u64>,
        state: BattleStateDto,
    },
    Snapshot {
        id: Option<u64>,
        snapshot: SnapshotDto,
    },
    Report {
        id: Option<u64>,
        report: RunReportDto,
    },
}
