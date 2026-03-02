// projects/products/unstable/hospital_tycoon/backend/src/sim/sim_event.rs
use crate::model::patient_id::PatientId;
use crate::model::room_id::RoomId;
use crate::time::tick::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimEvent {
    pub tick: Tick,
    pub kind: SimEventKind,
}

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

impl SimEvent {
    pub fn patient_arrived(tick: Tick, patient_id: PatientId) -> Self {
        Self {
            tick,
            kind: SimEventKind::PatientArrived { patient_id },
        }
    }
    pub fn patient_assigned(tick: Tick, patient_id: PatientId, room_id: RoomId) -> Self {
        Self {
            tick,
            kind: SimEventKind::PatientAssigned {
                patient_id,
                room_id,
            },
        }
    }
    pub fn patient_treated(tick: Tick, patient_id: PatientId) -> Self {
        Self {
            tick,
            kind: SimEventKind::PatientTreated { patient_id },
        }
    }
    pub fn patient_discharged(tick: Tick, patient_id: PatientId) -> Self {
        Self {
            tick,
            kind: SimEventKind::PatientDischarged { patient_id },
        }
    }
    pub fn budget_updated(tick: Tick, balance: i64) -> Self {
        Self {
            tick,
            kind: SimEventKind::BudgetUpdated { balance },
        }
    }
    pub fn reputation_changed(tick: Tick, score: u32) -> Self {
        Self {
            tick,
            kind: SimEventKind::ReputationChanged { score },
        }
    }
}
