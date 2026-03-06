// projects/products/unstable/hospital_tycoon/backend/src/sim/sim_event_kind.rs
use crate::model::patient_id::PatientId;
use crate::model::room_id::RoomId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimEventKind {
    PatientArrived {
        patient_id: PatientId,
    },
    PatientAssigned {
        patient_id: PatientId,
        room_id: RoomId,
    },
    PatientTreated {
        patient_id: PatientId,
    },
    PatientDischarged {
        patient_id: PatientId,
    },
    BudgetUpdated {
        balance: i64,
    },
    ReputationChanged {
        score: u32,
    },
}
