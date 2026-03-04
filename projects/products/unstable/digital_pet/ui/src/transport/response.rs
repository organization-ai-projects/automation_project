use crate::transport::pet_state_dto::PetStateDto;
use crate::transport::run_report_dto::RunReportDto;
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
    PetState {
        id: Option<u64>,
        state: PetStateDto,
    },
    Report {
        id: Option<u64>,
        report: RunReportDto,
    },
}
