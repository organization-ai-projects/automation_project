// projects/products/unstable/digital_pet/backend/src/training/training_engine.rs
use crate::model::pet::Pet;
use crate::time::tick::Tick;
use crate::training::training_kind::TrainingKind;
use crate::training::training_result::TrainingResult;

pub struct TrainingEngine;

impl TrainingEngine {
    pub fn new() -> Self { Self }

    pub fn train(&mut self, pet: &mut Pet, kind: &str, tick: Tick) -> TrainingResult {
        let _ = tick;
        let kind = TrainingKind::from_str(kind);
        let gain = 2u32;
        match kind {
            TrainingKind::Strength => pet.attack += gain,
            TrainingKind::Speed => pet.attack += gain / 2,
            TrainingKind::Defense => pet.defense += gain,
            TrainingKind::Stamina => { pet.max_hp += gain; pet.hp = pet.hp.min(pet.max_hp); }
        }
        TrainingResult { kind, stat_gain: gain }
    }
}
