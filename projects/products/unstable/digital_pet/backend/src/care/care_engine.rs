// projects/products/unstable/digital_pet/backend/src/care/care_engine.rs
use crate::care::care_action::CareAction;
use crate::care::care_mistake::CareMistake;
use crate::needs::needs_state::NeedsState;
use crate::time::tick::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CareEngine {
    pub actions: Vec<CareAction>,
    pub mistakes: Vec<CareMistake>,
    pub last_overhunger_tick: Option<Tick>,
    pub last_overfatigue_tick: Option<Tick>,
}

impl CareEngine {
    pub fn new() -> Self {
        Self { actions: vec![], mistakes: vec![], last_overhunger_tick: None, last_overfatigue_tick: None }
    }

    pub fn apply_action(&mut self, kind: String, needs: &mut NeedsState, tick: Tick) {
        match kind.as_str() {
            "feed" => needs.feed(),
            "rest" => needs.rest(),
            "play" => needs.play(),
            "discipline" => needs.discipline(),
            "medicine" => needs.medicine(),
            _ => {}
        }
        self.actions.push(CareAction { kind, tick });
    }

    pub fn evaluate(&mut self, needs: &NeedsState, tick: Tick) {
        if needs.hunger > 80 {
            if self.last_overhunger_tick.map_or(true, |t| tick.value() > t.value() + 10) {
                self.mistakes.push(CareMistake { reason: "neglected_hunger".into(), tick });
                self.last_overhunger_tick = Some(tick);
            }
        }
        if needs.fatigue > 80 {
            if self.last_overfatigue_tick.map_or(true, |t| tick.value() > t.value() + 10) {
                self.mistakes.push(CareMistake { reason: "neglected_fatigue".into(), tick });
                self.last_overfatigue_tick = Some(tick);
            }
        }
    }

    pub fn mistake_count(&self) -> usize { self.mistakes.len() }
}
