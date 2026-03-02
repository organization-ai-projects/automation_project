// projects/products/unstable/hospital_tycoon/backend/src/reputation/reputation_engine.rs
use crate::reputation::reputation::Reputation;

pub struct ReputationEngine;

impl ReputationEngine {
    pub fn on_patient_treated(reputation: &mut Reputation) {
        reputation.increase(1);
    }

    pub fn on_patient_died(reputation: &mut Reputation) {
        reputation.decrease(5);
    }
}
