// projects/products/unstable/hospital_tycoon/backend/src/economy/economy_engine.rs
use crate::economy::budget::Budget;
use crate::economy::pricing::Pricing;

pub struct EconomyEngine {
    pub pricing: Pricing,
}

impl EconomyEngine {
    pub fn new(pricing: Pricing) -> Self {
        Self { pricing }
    }

    pub fn on_patient_treated(&self, budget: &mut Budget) {
        budget.add_income(self.pricing.consultation_fee + self.pricing.treatment_fee);
    }
}
